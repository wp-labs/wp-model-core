# wp-model-core

![CI](https://github.com/wp-labs/wp-model-core/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/wp-labs/wp-model-core/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-model-core)
![License](https://img.shields.io/badge/License-Elastic%202.0-green.svg)

## Overview
`wp-model-core` is the typed data model that powers the Warp PASE stack. It defines the field, record, and value abstractions that upstream services rely on to validate input, serialize payloads, and reason about metadata in a consistent way. The crate ships as a library only and leans on familiar Rust tooling (`serde`, `chrono`, `ipnet`, `smol_str`) for ergonomics.

## Features
- 20+ value variants (`Value::Bool`, `Value::Email`, `Value::IpNet`, etc.) backed by lightweight `smol_str` storage for short text fields.
- Rich metadata through `DataType`, ensuring every value carries run-time constraints (format, semantic type, array hints).
- Field and record containers with ergonomic builders (`Field::from_digit`, `Record::set_id`) plus shared aliases (`DataField`, `DataRecord`).
- Serde integration and deterministic formatting helpers (`LevelFormatAble`) for debugging or downstream logging.
- `thiserror` based error types that keep callers away from `unwrap`/`expect` paths.

## Project Layout
- `src/lib.rs` exposes `model` and `traits` modules.
- `src/model/` hosts the value system, field/record implementations, formatting helpers, macros, and the `README.md` that documents the design.
- `src/traits.rs` provides shared traits such as `AsValueRef` consumed across models.
- `docs/` holds supplementary specs; `AGENTS.md` documents contributor expectations.

## Quick Start
```toml
# Cargo.toml
dependencies:
  wp-model-core = "0.7"
```
```rust
use wp_model_core::model::{DataField, DataRecord, DataType, Field, Value};

let mut record = DataRecord::default();
record.append(Field::from_digit("count", 42));
record.append(Field::from_chars("status", "ok"));
assert_eq!(record.get_value("count"), Some(&Value::Digit(42)));
```

## Development Workflow
Run `cargo check` for quick iteration, `cargo build` for release targets, and `cargo test` to execute the co-located unit suites (see `src/model/data/*`). Format with `cargo fmt --all`, lint with `cargo clippy --all-targets --all-features -D warnings`, and regenerate docs via `cargo doc --open` when changing public APIs.

## Testing & QA
Tests live beside their modules (`mod tests` blocks). Name tests `test_*` and focus on serialization, comparison, and metadata coverage when introducing a new value type or record behavior. For fuzz targets, lean on the `wp-data-model` workspace where fuzz harnesses reside.

## Documentation & Support
- Architecture notes: `src/model/README.md`
- Contributor guide: `AGENTS.md`
- Issue tracking and releases: GitHub [`wp-labs/wp-model-core`](https://github.com/wp-labs/wp-model-core)

## License
Distributed under the Elastic License 2.0. See [`LICENSE`](LICENSE) for details.
