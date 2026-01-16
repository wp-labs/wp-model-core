# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/wp-labs/wp-model-core/compare/v0.7.1...HEAD
[0.7.1]: https://github.com/wp-labs/wp-model-core/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/wp-labs/wp-model-core/releases/tag/v0.7.0
