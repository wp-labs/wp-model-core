# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.2] - 2026-02-07

### Added

- Add `id: u64` field to `Record<T>` struct for direct access to record ID
- Update `set_id()` method to set both the new `id` field and maintain backward compatibility by storing ID in items collection as `wp_event_id`

## [0.7.1] - 2026-01-16

### Added

- Add `KvArr` variant to `DataType` enum for key-value array type support
- Add `KVARR` constant with value `"kvarr"`
- Add serde serialization support for `KvArr` with rename `"kvarr"`

## [0.7.0] - 2026-01-15

### Added

- Initial release of wp-model-core
- `DataType` enum with comprehensive type definitions (Bool, Chars, Symbol, Digit, Float, Time variants, IP, Domain, Email, etc.)
- `Value`, `Field`, and `Record` primitives for the Warp PASE stack
- Serde serialization/deserialization support
- Time format aliases (CLF, RFC3339, RFC2822, timestamp)
- HTTP type support (request, status, agent, method)
- Array type with subtype specification

[Unreleased]: https://github.com/wp-labs/wp-model-core/compare/v0.7.2...HEAD
[0.7.2]: https://github.com/wp-labs/wp-model-core/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/wp-labs/wp-model-core/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/wp-labs/wp-model-core/releases/tag/v0.7.0
