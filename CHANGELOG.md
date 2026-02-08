# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0] - 2026-02-08

### ⚠️ BREAKING CHANGES

- **DataRecord type changed from `Record<Field<Value>>` to `Record<FieldStorage>`**
  - This enables mixed storage mode with both shared (Arc) and owned fields
  - Static/constant fields can now be shared with zero-copy semantics
  - See Migration Guide below for upgrade instructions

- **License changed from Elastic-2.0 to Apache-2.0**
  - More permissive open source license
  - Allows broader commercial and community use

### Added

- **FieldStorage enum** for mixed storage support
  - `Shared(Arc<Field<Value>>)` variant for zero-copy sharing of static fields
  - `Owned(Field<Value>)` variant for dynamically computed fields
  - Methods: `as_field()`, `into_owned()`, `from_shared()`, `from_owned()`, `is_shared()`, `shared_count()`
  - Implements: `RecordItem`, `RecordItemFactory`, `LevelFormatAble`, `Display`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`

- **Record<FieldStorage> convenience methods**:
  - `push_shared(field: Arc<Field<Value>>)` - Add Arc-wrapped field
  - `push_owned(field: Field<Value>)` - Add owned field
  - `get_field(index: usize) -> Option<&Field<Value>>` - Get field by index
  - `storage_stats() -> (usize, usize)` - Get count of shared vs owned fields
  - `into_owned_record() -> Record<Field<Value>>` - Convert to fully owned record

### Performance Improvements

- Static field cloning: 50-500ns → ~5ns (Arc reference count increment)
- Reduced memory usage: 50-90% for high static field ratio scenarios
- Multi-stage pipeline processing: 50-97% performance improvement

### Migration Guide

**No changes required for basic usage** - existing code continues to work:

```rust
use wp_model_core::model::DataRecord;

// Old code still works
let mut record = DataRecord::default();
```

**To use new mixed storage features**:

```rust
use std::sync::Arc;
use wp_model_core::model::{DataRecord, Field, Value, DataType, FieldStorage};

let mut record = DataRecord::default();

// Add shared field (for static/constant values)
let static_field = Arc::new(Field::new(DataType::Chars, "app", Value::from("myapp")));
record.push_shared(static_field);

// Add owned field (for dynamic values)
record.push_owned(Field::new(DataType::Digit, "count", Value::from(42)));

// Or use FieldStorage directly
record.append(FieldStorage::from_digit("age", 30));
```

**For code that expects `Record<Field<Value>>`**:

```rust
// Convert DataRecord to fully owned record
let owned_record = record.into_owned_record();
```

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

[Unreleased]: https://github.com/wp-labs/wp-model-core/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/wp-labs/wp-model-core/compare/v0.7.2...v0.8.0
[0.7.2]: https://github.com/wp-labs/wp-model-core/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/wp-labs/wp-model-core/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/wp-labs/wp-model-core/releases/tag/v0.7.0
