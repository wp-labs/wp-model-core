use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{process, thread};

static WP_EVENT_ID_SEED: OnceLock<AtomicU64> = OnceLock::new();

/// Returns a process-local monotonically increasing `wp_event_id`.
///
/// The initial seed mixes wall-clock time, process identity, and runtime entropy
/// so the sequence does not fall back to a fixed starting value after restart.
pub fn next_wp_event_id() -> u64 {
    WP_EVENT_ID_SEED
        .get_or_init(|| AtomicU64::new(init_wp_event_id_seed()))
        .fetch_add(1, Ordering::Relaxed)
}

fn init_wp_event_id_seed() -> u64 {
    compose_wp_event_id_seed(
        SystemTime::now().duration_since(UNIX_EPOCH).ok(),
        process::id(),
        runtime_entropy(),
    )
}

fn compose_wp_event_id_seed(time_since_epoch: Option<Duration>, pid: u32, entropy: u64) -> u64 {
    let time_nanos = time_since_epoch.map(duration_to_u64_nanos).unwrap_or(0);
    let pid_bits = u64::from(pid).rotate_left(32);

    let mut seed = entropy ^ time_nanos.rotate_left(13) ^ pid_bits ^ 0x9E37_79B9_7F4A_7C15;

    seed ^= seed >> 33;
    seed = seed.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    seed ^= seed >> 33;
    seed = seed.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
    seed ^= seed >> 33;

    if seed == 0 { 1 } else { seed }
}

fn duration_to_u64_nanos(duration: Duration) -> u64 {
    duration.as_nanos() as u64
}

fn runtime_entropy() -> u64 {
    let mut stack_marker = 0_u8;
    let stack_addr = (&mut stack_marker as *mut u8 as usize) as u64;
    let thread_id_hash = hash_thread_id(thread::current().id());

    let mut hasher = RandomState::new().build_hasher();
    process::id().hash(&mut hasher);
    stack_addr.hash(&mut hasher);
    thread_id_hash.hash(&mut hasher);

    hasher.finish() ^ thread_id_hash.rotate_left(17) ^ stack_addr.rotate_left(29)
}

fn hash_thread_id(thread_id: thread::ThreadId) -> u64 {
    let mut hasher = DefaultHasher::new();
    thread_id.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_wp_event_id_seed_is_non_zero_when_time_is_unavailable() {
        let seed = compose_wp_event_id_seed(None, 42, 0x1234_5678_9ABC_DEF0);
        assert_ne!(seed, 0);
    }

    #[test]
    fn compose_wp_event_id_seed_changes_across_restarts_even_without_time() {
        let first = compose_wp_event_id_seed(None, 42, 0x1111_2222_3333_4444);
        let second = compose_wp_event_id_seed(None, 42, 0x5555_6666_7777_8888);
        assert_ne!(first, second);
    }

    #[test]
    fn compose_wp_event_id_seed_changes_with_same_entropy_but_different_time() {
        let first = compose_wp_event_id_seed(Some(Duration::from_secs(1)), 42, 7);
        let second = compose_wp_event_id_seed(Some(Duration::from_secs(2)), 42, 7);
        assert_ne!(first, second);
    }

    #[test]
    fn next_wp_event_id_is_monotonic_within_one_process() {
        let first = next_wp_event_id();
        let second = next_wp_event_id();
        assert_eq!(second, first + 1);
    }
}
