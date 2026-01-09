# Repository Guidelines

## Project Structure & Module Organization
`wp-model-core` is a single Rust library crate. Core exports live in `src/lib.rs`, domain models under `src/model/`, and reusable behaviors inside `src/traits.rs`. Module-level docs (for example `src/model/README.md`) capture design notes and should be kept in sync with the code. Unit tests sit beside their targets via `#[cfg(test)] mod tests`, so keep new scenarios close to the implementation they verify. Workspace assets such as specs or diagrams belong in `docs/`, while generated build outputs remain under `target/`.

## Build, Test, and Development Commands
Use `cargo check` for fast type validation during iteration, `cargo build` for a full release-ready compile, and `cargo test` to run the co-located unit suite. Formatting and linting are mandatory before sending changes: `cargo fmt --all` enforces consistent layout and `cargo clippy --all-targets --all-features -D warnings` keeps the public API tidy. `cargo doc --open` is handy when reviewing trait contracts or complex generics.

## Coding Style & Naming Conventions
The crate targets Rust 2024 with the default `rustfmt` profile (4-space indent, 100-column guidance). Follow idiomatic naming: types and traits in `CamelCase`, modules/functions in `snake_case`, constants in `SCREAMING_SNAKE_CASE`. Prefer explicit `Result<T, Error>` flows surfaced through the shared `thiserror` enums rather than `unwrap`/`expect`. Public APIs need `///` docs, and crate/module headers should start with `//!` narratives.

## Testing Guidelines
Add new tests inside the relevant module, naming them `test_*` for quick `cargo test test_field_builder` filters. Favor lightweight unit tests that assert serialization (`serde`) and comparison behavior because higher layers consume this crate as a dependency. When adding value or metadata types, mirror the existing pattern in `src/model/types/` and extend the associated tests before touching downstream callers.

## Commit & Pull Request Guidelines
Write Conventional Commits with an optional crate scope, e.g., `feat(model): add mac address datatype`. Each PR should describe the motivation, link any tracking issue, and call out schema or serialization changes. Attach `cargo fmt`, `cargo clippy`, and `cargo test` results (or explain gaps) so reviewers can focus on semantics.

## Security & Configuration Tips
Never check in sample secrets or production payloads. Treat all inputs as untrusted; rely on the provided validator helpers and return meaningful errors instead of panicking. Keep dependency versions aligned with `Cargo.lock`, and avoid ad-hoc feature flags without documenting them in `docs/`.
