# wp-model-core

[![Crates.io](https://img.shields.io/crates/v/wp-model-core.svg)](https://crates.io/crates/wp-model-core)
[![Docs.rs](https://docs.rs/wp-model-core/badge.svg)](https://docs.rs/wp-model-core)
[![CI](https://img.shields.io/github/actions/workflow/status/wp-labs/wp-model-core/ci.yml?branch=main)](https://github.com/wp-labs/wp-model-core/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/wp-labs/wp-model-core/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-model-core)
[![Crates.io downloads](https://img.shields.io/crates/d/wp-model-core)](https://crates.io/crates/wp-model-core)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust Edition](https://img.shields.io/badge/edition-2024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

## Overview
`wp-model-core` is the typed data model that powers the Warp PASE stack. It defines the field, record, and value abstractions that upstream services rely on to validate input, serialize payloads, and reason about metadata in a consistent way. The crate ships as a library only and leans on familiar Rust tooling (`serde`, `chrono`, `num-bigint`, `smol_str`) for ergonomics.

## Features
- 20+ value variants (`Value::Bool`, `Value::Email`, `Value::IpNet`, etc.) backed by lightweight `smol_str` storage for short text fields.
- Rich metadata through `DataType`, ensuring every value carries run-time constraints (format, semantic type, array hints).
- Field and record containers with ergonomic builders (`Field::from_int`, `Record::set_id`) plus shared aliases (`DataField`, `DataRecord`).
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
  wp-model-core = "0.10"
```
```rust
use wp_model_core::model::{DataField, DataRecord, DataType, Field, Value};

let mut record = DataRecord::default();
record.append(Field::from_int("count", 42));
record.append(Field::from_chars("status", "ok"));
assert_eq!(record.get_value("count"), Some(&Value::Int(42)));
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
Distributed under the Apache License 2.0. See [`LICENSE`](LICENSE) for details.
