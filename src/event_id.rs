//! `wp_event_id` 生成器：`secs(32) | instance_lane(10) | counter(22)`。
//!
//! ## 布局
//!
//! ```text
//!  63            32 31        22 21                    0
//! +----------------+-----------+-----------------------+
//! |  秒 (32 bit)   | 实例(10)  |  每秒计数器 (22 bit)  |
//! +----------------+-----------+-----------------------+
//! ```
//!
//! - **秒**：Unix 时间戳（1970 起算），到 2106-02-07；高位在前，`ORDER BY wp_event_id`
//!   即时间序。
//! - **实例号 lane**：`instance_id - INS_ID_BASE`，编号从 `100` 起，可用区间
//!   `100..=1123`（共 1024 个实例）。
//! - **每秒计数器**：`0..=4_194_303`，每秒归零；超出时逻辑秒前进一格，绝不回绕。
//!
//! 实例号由环境变量 `WP_INS_ID` 提供（也可写在 work root 的 `.env`，与
//! `WP_GLOBAL_TAGS` 走同一套加载逻辑）；未设置时使用默认值 `100`。
//!
//! ## 为什么这么分
//!
//! 单实例压测可达 200 万 EPS，每秒钟计数器至少要 22 位
//! （`2^22 = 4,194,304`，2.1 倍余量）；秒级时间戳要覆盖到 2106 年需要 32 位。
//! `64 - 22 - 32 = 10` 位留给实例号，正好 1024 个实例。
//!
//! ## 冲突边界
//!
//! 唯一性由 **实例号** 保证：只要每个实例拿到不同的 `WP_INS_ID`，跨实例永不重复。
//!
//! 已在生成器内部处理的情形：
//!
//! - **同一秒内重启**：启动时把逻辑秒对齐到「下一整秒」，不会重放上一轮用过的秒。
//! - **挂钟回拨**：沿用已用过的逻辑秒继续累加计数器，不会倒退。
//! - **单秒超量**（> 4,194,304 条）：逻辑秒前进一格，不回绕。
//!
//! 仍可能重复、只能靠部署侧避免的情形：
//!
//! - 两个实例配置了相同的 `WP_INS_ID`（包括都没配置、都回退到默认 `100`）。
//! - 单秒超量导致逻辑秒前移后，在挂钟追上之前又重启：极小概率与上一轮重叠。
//! - `fork()` 后不 `exec` 的子进程会共享父进程的计数器状态（`exec` 会重建，不受影响）。

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// 实例号环境变量。也可写在 work root 的 `.env`（与 `WP_GLOBAL_TAGS` 同一套逻辑）。
pub const WP_INS_ID_ENV: &str = "WP_INS_ID";

/// 实例号编号起点。
pub const INS_ID_BASE: u16 = 100;

/// 每秒计数器位宽。
pub const COUNTER_BITS: u32 = 22;
/// 实例号（lane）位宽。
pub const LANE_BITS: u32 = 10;
/// 秒级时间戳位宽。
pub const SECS_BITS: u32 = 32;

/// 每秒计数器上界（含）。
pub const MAX_COUNTER: u32 = (1u32 << COUNTER_BITS) - 1;
/// lane 上界（含）。
pub const MAX_LANE: u16 = (1u16 << LANE_BITS) - 1;
/// 可用实例号上界（含）。
pub const INS_ID_MAX: u16 = INS_ID_BASE + MAX_LANE;
/// 时间戳上界（含），对应 2106-02-07。
pub const MAX_SECS: u64 = (1u64 << SECS_BITS) - 1;

const LANE_SHIFT: u32 = COUNTER_BITS;
const SECS_SHIFT: u32 = COUNTER_BITS + LANE_BITS;

/// 打包后的 `(secs << COUNTER_BITS) | counter`；`0` 表示「尚未发号」。
static STATE: AtomicU64 = AtomicU64::new(0);
static LANE: OnceLock<u16> = OnceLock::new();
/// 本进程允许使用的最早逻辑秒，见 [`next_wp_event_id`]。
static MIN_SECS: OnceLock<u64> = OnceLock::new();

/// 返回进程内严格单调递增的 `wp_event_id`。
///
/// 启动后的第一条 ID 落在「首次调用所在的整秒 + 1」，用来避开同秒重启重放；
/// 之后跟随挂钟走。挂钟回拨时沿用已用过的逻辑秒并继续累加计数器，因此不会重复。
pub fn next_wp_event_id() -> u64 {
    let lane = resolved_lane();
    // 启动对齐：把「首次发号所在的秒」整秒跳过，避免同秒重启重放上一轮的号。
    let min_secs = *MIN_SECS.get_or_init(|| align_start_secs(unix_secs()));

    loop {
        let cur = STATE.load(Ordering::Relaxed);
        let wall = unix_secs().max(min_secs);
        let (next, secs, counter) = advance(cur, wall);
        if secs > MAX_SECS {
            panic!(
                "wp_event_id 时间戳溢出：秒数 {secs} 超过 {SECS_BITS} 位上界 {MAX_SECS}（2106-02-07）"
            );
        }
        if STATE
            .compare_exchange_weak(cur, next, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            return compose_event_id(secs, lane, counter);
        }
    }
}

/// 当前生效的实例号（`INS_ID_BASE..=INS_ID_MAX`）。
pub fn instance_id() -> u16 {
    INS_ID_BASE + resolved_lane()
}

/// 从 `WP_INS_ID` 初始化实例号。
///
/// - `Ok(Some(id))`：环境变量已设置且合法。
/// - `Ok(None)`：未设置，回退默认值 [`INS_ID_BASE`]。
/// - `Err(_)`：值非法（非数字或越界），调用方应终止启动。
///
/// 若实例号在首次发号后被再次初始化且取值不同，返回错误 —— 一个进程只能有一个实例号。
pub fn init_instance_id_from_env() -> Result<Option<u16>, String> {
    let raw = std::env::var(WP_INS_ID_ENV).ok();
    let explicit = is_explicit(raw.as_deref());
    let id = parse_instance_id(raw.as_deref())?;
    let lane = id - INS_ID_BASE;

    if LANE.set(lane).is_err() {
        let current = INS_ID_BASE + *LANE.get().expect("lane initialized");
        if current != id {
            return Err(format!(
                "{WP_INS_ID_ENV} 在首次发号后被改为 {id}（已生效 {current}）"
            ));
        }
    }

    Ok(if explicit { Some(id) } else { None })
}

/// 环境变量是否真的提供了值（`None` / 全空白 视为未提供）。
fn is_explicit(raw: Option<&str>) -> bool {
    matches!(raw.map(str::trim), Some(value) if !value.is_empty())
}

/// 解析实例号字符串；`None` 或空白返回默认值 [`INS_ID_BASE`]。
pub fn parse_instance_id(raw: Option<&str>) -> Result<u16, String> {
    let Some(raw) = raw else {
        return Ok(INS_ID_BASE);
    };
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(INS_ID_BASE);
    }
    let id: u32 = raw
        .parse()
        .map_err(|_| format!("{WP_INS_ID_ENV} 必须是整数，当前为 '{raw}'"))?;
    if id < u32::from(INS_ID_BASE) || id > u32::from(INS_ID_MAX) {
        return Err(format!(
            "{WP_INS_ID_ENV} 必须在 [{INS_ID_BASE}, {INS_ID_MAX}] 之间，当前为 {id}"
        ));
    }
    Ok(id as u16)
}

/// 按布局拼装 `wp_event_id`。
pub fn compose_event_id(secs: u64, lane: u16, counter: u32) -> u64 {
    debug_assert!(secs <= MAX_SECS, "secs out of range: {secs}");
    debug_assert!(lane <= MAX_LANE, "lane out of range: {lane}");
    debug_assert!(counter <= MAX_COUNTER, "counter out of range: {counter}");
    (secs << SECS_SHIFT) | (u64::from(lane) << LANE_SHIFT) | u64::from(counter)
}

/// 按布局拆解 `wp_event_id`，返回 `(secs, lane, counter)`。
pub fn decompose_event_id(id: u64) -> (u64, u16, u32) {
    let secs = id >> SECS_SHIFT;
    let lane = ((id >> LANE_SHIFT) & u64::from(MAX_LANE)) as u16;
    let counter = (id & u64::from(MAX_COUNTER)) as u32;
    (secs, lane, counter)
}

/// 推进一格打包状态，返回 `(新状态, 逻辑秒, 计数器)`。
///
/// - `wall > last_secs`：进入新的一秒，计数器归零。
/// - `wall <= last_secs`：沿用逻辑秒（挂钟回拨保护），计数器 +1。
/// - 计数器用尽：逻辑秒前进一格，保证不重复。
fn advance(cur: u64, wall: u64) -> (u64, u64, u32) {
    let last_secs = cur >> COUNTER_BITS;
    let last_counter = (cur & u64::from(MAX_COUNTER)) as u32;

    let (secs, counter) = if wall > last_secs {
        (wall, 0)
    } else if last_counter < MAX_COUNTER {
        (last_secs, last_counter + 1)
    } else {
        (last_secs + 1, 0)
    };

    ((secs << COUNTER_BITS) | u64::from(counter), secs, counter)
}

fn resolved_lane() -> u16 {
    *LANE.get_or_init(
        || match parse_instance_id(std::env::var(WP_INS_ID_ENV).ok().as_deref()) {
            Ok(id) => id - INS_ID_BASE,
            // 非法实例号宁可 panic，也不能静默回退到默认值：多个实例共用同一个 lane
            // 会持续产出重复 ID，比启动失败更危险。启动路径应先用
            // `init_instance_id_from_env()` 拿到可上报的错误。
            Err(err) => panic!("{err}"),
        },
    )
}

/// 启动对齐：首个可用逻辑秒 = 当前整秒 + 1。
///
/// 挂钟只向前走，所以上一轮进程用过的最后一个秒 ≤ 当前整秒。
/// 跳过当前整秒后，新进程不可能与上一轮重叠。
fn align_start_secs(now: u64) -> u64 {
    now.saturating_add(1)
}

fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_covers_exactly_64_bits() {
        assert_eq!(SECS_BITS + LANE_BITS + COUNTER_BITS, 64);
        assert_eq!(SECS_SHIFT, 32);
        assert_eq!(LANE_SHIFT, 22);
    }

    #[test]
    fn instance_id_range_matches_lane_width() {
        assert_eq!(INS_ID_BASE, 100);
        assert_eq!(MAX_LANE, 1023);
        assert_eq!(INS_ID_MAX, 1123);
        assert_eq!(usize::from(INS_ID_MAX - INS_ID_BASE) + 1, 1024);
    }

    #[test]
    fn compose_and_decompose_round_trip() {
        let id = compose_event_id(1_794_614_400, 5, 42);
        assert_eq!(decompose_event_id(id), (1_794_614_400, 5, 42));
    }

    #[test]
    fn compose_keeps_time_in_high_bits() {
        let early = compose_event_id(1_794_614_400, MAX_LANE, MAX_COUNTER);
        let later = compose_event_id(1_794_614_401, 0, 0);
        assert!(later > early, "时间高位必须主导排序");
    }

    #[test]
    fn max_composed_value_fits_u64_and_orders_by_time() {
        let max = compose_event_id(MAX_SECS, MAX_LANE, MAX_COUNTER);
        assert_eq!(max, u64::MAX);
        // 2026 年时仍在 i64 正区间内（2038 年会被越过）
        let now = compose_event_id(1_794_614_400, MAX_LANE, MAX_COUNTER);
        assert!(now < i64::MAX as u64);
    }

    #[test]
    fn counter_increments_within_same_second() {
        let (state, secs, counter) = advance(0, 1000);
        assert_eq!((secs, counter), (1000, 0));
        let (state, secs, counter) = advance(state, 1000);
        assert_eq!((secs, counter), (1000, 1));
        let (_, secs, counter) = advance(state, 1000);
        assert_eq!((secs, counter), (1000, 2));
    }

    #[test]
    fn counter_resets_when_wall_advances() {
        let (state, _, _) = advance(0, 1000);
        let (state, _, _) = advance(state, 1000);
        let (state, secs, counter) = advance(state, 1001);
        assert_eq!((secs, counter), (1001, 0));
        let (_, secs, counter) = advance(state, 1001);
        assert_eq!((secs, counter), (1001, 1));
    }

    #[test]
    fn clock_rollback_keeps_logical_second() {
        let (state, _, _) = advance(0, 1000);
        // 挂钟回拨到 900：必须沿用 1000 并继续累加，不能重放
        let (_, secs, counter) = advance(state, 900);
        assert_eq!((secs, counter), (1000, 1));
    }

    #[test]
    fn counter_overflow_advances_logical_second() {
        let full = (1000u64 << COUNTER_BITS) | u64::from(MAX_COUNTER);
        let (_, secs, counter) = advance(full, 1000);
        assert_eq!((secs, counter), (1001, 0));

        let (_, secs, counter) = advance(full, 1000 + 5);
        assert_eq!((secs, counter), (1000 + 5, 0), "挂钟已前进时直接取挂钟");
    }

    #[test]
    fn advance_is_monotonic_including_overflow() {
        let mut state = 0u64;
        let mut prev = 0u64;
        for _ in 0..10 {
            let (next, secs, counter) = advance(state, 1000);
            let id = compose_event_id(secs, 1, counter);
            assert!(id > prev, "id must increase: {prev} -> {id}");
            prev = id;
            state = next;
        }
        // 直接把计数器推到上界，验证溢出后仍然递增
        let full = (1000u64 << COUNTER_BITS) | u64::from(MAX_COUNTER);
        let (_, secs, counter) = advance(full, 1000);
        assert!(compose_event_id(secs, 1, counter) > compose_event_id(1000, 1, MAX_COUNTER));
    }

    #[test]
    fn next_wp_event_id_is_monotonic() {
        let first = next_wp_event_id();
        let second = next_wp_event_id();
        assert!(second > first, "{first} -> {second}");
    }

    #[test]
    fn next_wp_event_id_never_uses_a_past_second() {
        let before = unix_secs();
        let (secs, _, _) = decompose_event_id(next_wp_event_id());
        assert!(
            secs >= before,
            "逻辑秒不得落后于挂钟: before={before} secs={secs}"
        );
        assert!(secs > 0);
    }

    #[test]
    fn parse_instance_id_defaults_to_base() {
        assert_eq!(parse_instance_id(None).unwrap(), INS_ID_BASE);
        assert_eq!(parse_instance_id(Some("")).unwrap(), INS_ID_BASE);
        assert_eq!(parse_instance_id(Some("   ")).unwrap(), INS_ID_BASE);
    }

    #[test]
    fn parse_instance_id_accepts_range() {
        assert_eq!(parse_instance_id(Some("100")).unwrap(), 100);
        assert_eq!(parse_instance_id(Some(" 512 ")).unwrap(), 512);
        assert_eq!(parse_instance_id(Some("1123")).unwrap(), 1123);
    }

    #[test]
    fn parse_instance_id_rejects_out_of_range() {
        assert!(parse_instance_id(Some("99")).is_err());
        assert!(parse_instance_id(Some("1124")).is_err());
        assert!(parse_instance_id(Some("0")).is_err());
        assert!(parse_instance_id(Some("-1")).is_err());
    }

    #[test]
    fn parse_instance_id_rejects_non_numeric() {
        let err = parse_instance_id(Some("abc")).unwrap_err();
        assert!(err.contains(WP_INS_ID_ENV), "{err}");
    }

    #[test]
    fn start_alignment_skips_the_current_second() {
        assert_eq!(align_start_secs(1_000), 1_001);
        assert_eq!(align_start_secs(0), 1);
        // 上界不 panic（2106 年后会先撞到 MAX_SECS 检查）
        assert_eq!(align_start_secs(u64::MAX), u64::MAX);
    }

    #[test]
    fn is_explicit_requires_a_non_blank_value() {
        assert!(!is_explicit(None));
        assert!(!is_explicit(Some("")));
        assert!(!is_explicit(Some("   ")));
        assert!(!is_explicit(Some("\t")));
        assert!(is_explicit(Some("100")));
        assert!(is_explicit(Some(" 100 ")));
    }

    #[test]
    fn lane_shifts_do_not_overlap() {
        let a = compose_event_id(7, 1000, 1);
        let (secs, lane, counter) = decompose_event_id(a);
        assert_eq!((secs, lane, counter), (7, 1000, 1));
    }

    #[test]
    fn different_lanes_never_collide_in_same_second() {
        let ids: Vec<u64> = (0..MAX_LANE)
            .map(|lane| compose_event_id(1_794_614_400, lane, 7))
            .collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len());
    }

    // ---- 容量底线：布局参数一旦改动，这些断言会立刻失败 ----

    #[test]
    fn counter_capacity_keeps_two_times_headroom_over_2m_eps() {
        let max_eps = u64::from(MAX_COUNTER) + 1;
        assert_eq!(max_eps, 4_194_304, "每秒计数器容量变了");
        assert!(max_eps >= 2 * 2_000_000, "200 万 EPS 至少需要 2 倍余量");
    }

    #[test]
    fn secs_field_covers_at_least_75_years() {
        let now_2026 = 1_794_614_400u64;
        let seventy_five_years = 75 * 365 * 24 * 3600;
        assert!(now_2026 < MAX_SECS);
        assert!(
            now_2026 + seventy_five_years < MAX_SECS,
            "32 位秒应至少覆盖到 2101 年"
        );
    }

    #[test]
    fn instance_capacity_is_1024() {
        assert_eq!(u32::from(MAX_LANE) + 1, 1024);
        assert_eq!(INS_ID_MAX - INS_ID_BASE + 1, 1024);
    }

    // ---- 布局边界 ----

    #[test]
    fn decompose_handles_layout_extremes() {
        assert_eq!(decompose_event_id(0), (0, 0, 0));
        assert_eq!(
            decompose_event_id(u64::MAX),
            (MAX_SECS, MAX_LANE, MAX_COUNTER)
        );
    }

    #[test]
    fn compose_of_zero_is_zero() {
        assert_eq!(compose_event_id(0, 0, 0), 0);
    }

    #[test]
    fn every_layout_field_is_isolated() {
        // 只改一个字段不能污染其它字段
        let base = compose_event_id(1_000, 0, 0);
        assert_eq!(decompose_event_id(base), (1_000, 0, 0));

        let with_lane = compose_event_id(1_000, MAX_LANE, 0);
        assert_eq!(decompose_event_id(with_lane), (1_000, MAX_LANE, 0));

        let with_counter = compose_event_id(1_000, 0, MAX_COUNTER);
        assert_eq!(decompose_event_id(with_counter), (1_000, 0, MAX_COUNTER));
    }

    #[test]
    fn instance_id_to_lane_mapping_is_contiguous() {
        assert_eq!(parse_instance_id(Some("100")).unwrap() - INS_ID_BASE, 0);
        assert_eq!(parse_instance_id(Some("101")).unwrap() - INS_ID_BASE, 1);
        assert_eq!(
            parse_instance_id(Some("1123")).unwrap() - INS_ID_BASE,
            MAX_LANE
        );
    }

    #[test]
    fn parse_instance_id_is_strict_about_format() {
        // 容忍前导零与显式正号（Rust integer 解析的标准行为）
        assert_eq!(parse_instance_id(Some("0100")).unwrap(), 100);
        assert_eq!(parse_instance_id(Some("+100")).unwrap(), 100);
        // 十六进制、浮点、下划线分隔、全角数字都必须被拒
        assert!(parse_instance_id(Some("0x64")).is_err());
        assert!(parse_instance_id(Some("100.0")).is_err());
        assert!(parse_instance_id(Some("1_00")).is_err());
        assert!(parse_instance_id(Some("１００")).is_err());
    }

    // ---- 跨秒/回拨/并发 ----

    #[test]
    fn advance_never_repeats_across_second_boundaries() {
        let mut state = 0u64;
        let mut seen = std::collections::HashSet::new();
        for step in 0..60_000u64 {
            let wall = 1_000_000 + step / 20_000;
            let (next, secs, counter) = advance(state, wall);
            state = next;
            assert!(
                seen.insert((secs, counter)),
                "重复的 (secs, counter): step={step} secs={secs} counter={counter}"
            );
        }
    }

    #[test]
    fn advance_stays_monotonic_while_the_clock_jitters() {
        // 挂钟前进 + 回拨交替，ID 仍必须严格递增
        let walls = [
            1_000u64, 1_000, 1_001, 1_000, 999, 1_002, 1_001, 1_003, 1_003, 1_002,
        ];
        let mut state = 0u64;
        let mut prev: Option<u64> = None;
        for round in 0..50u32 {
            let wall = walls[round as usize % walls.len()];
            let (next, secs, counter) = advance(state, wall);
            state = next;
            let id = compose_event_id(secs, 3, counter);
            if let Some(previous) = prev {
                assert!(id > previous, "回拨后 ID 倒退了: {previous} -> {id}");
            }
            prev = Some(id);
        }
    }

    #[test]
    fn concurrent_generation_is_unique() {
        use std::collections::HashSet;
        use std::sync::{Arc, Mutex};

        const THREADS: usize = 8;
        const PER_THREAD: usize = 2_000;

        let sink = Arc::new(Mutex::new(HashSet::with_capacity(THREADS * PER_THREAD)));
        let mut handles = Vec::with_capacity(THREADS);
        for _ in 0..THREADS {
            let sink = Arc::clone(&sink);
            handles.push(std::thread::spawn(move || {
                let mut local = Vec::with_capacity(PER_THREAD);
                for _ in 0..PER_THREAD {
                    local.push(next_wp_event_id());
                }
                sink.lock().expect("sink lock").extend(local);
            }));
        }
        for handle in handles {
            handle.join().expect("worker join");
        }

        let seen = sink.lock().expect("sink lock");
        assert_eq!(seen.len(), THREADS * PER_THREAD, "并发发号出现重复");
    }

    #[test]
    fn generated_ids_all_carry_the_configured_lane() {
        let expected_lane = instance_id() - INS_ID_BASE;
        let mut previous_secs = 0u64;
        for _ in 0..1_000 {
            let (secs, lane, _) = decompose_event_id(next_wp_event_id());
            assert_eq!(lane, expected_lane, "lane 段被其它字段污染");
            assert!(secs >= previous_secs, "逻辑秒倒退了");
            previous_secs = secs;
        }
    }
}
