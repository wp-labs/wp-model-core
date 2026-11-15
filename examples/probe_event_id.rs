//! 打印 `wp_event_id` 的三段布局，用于核对实例号与时间戳。
//!
//! ```bash
//! cargo run --example probe_event_id        # 默认 5 条
//! cargo run --example probe_event_id -- 20  # 指定条数
//! WP_INS_ID=101 cargo run --example probe_event_id
//! ```
//!
//! 布局：`秒(32) | 实例号(10) | 每秒计数器(22)`

use wp_model_core::event_id::{decompose_event_id, instance_id, next_wp_event_id};

fn main() {
    let count: usize = std::env::args()
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(5);

    println!("instance_id = {} (WP_INS_ID)", instance_id());
    println!(
        "{:<22} {:>12} {:>6} {:>12}",
        "wp_event_id", "secs", "lane", "counter"
    );

    for _ in 0..count {
        let id = next_wp_event_id();
        let (secs, lane, counter) = decompose_event_id(id);
        println!("{id:<22} {secs:>12} {lane:>6} {counter:>12}");
    }
}
