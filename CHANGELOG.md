# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.6]

### Added

#### Event ID Generator

- New public `event_id` module exposing `next_wp_event_id()` for generating process-local monotonically increasing `wp_event_id` values
- Seed initialization now mixes wall-clock time, process ID, and runtime entropy so restarts do not fall back to a fixed starting value
- Unit tests cover monotonic generation within one process plus non-zero and variable seed composition when wall-clock time is unavailable

### Changed

- Align release automation with the library version admin flow in `_gal/adm.gxl`

## [0.8.5]

### Added

#### RawData Container

- New `raw` module with `RawData` enum — a unified raw data representation supporting multiple underlying storage backends
  - `RawData::String(String)` — UTF-8 string storage
  - `RawData::Bytes(Bytes)` — `bytes::Bytes` ref-counted storage, suitable for network I/O scenarios
  - `RawData::ArcBytes(Arc<Vec<u8>>)` — `Arc` shared byte storage, enables zero-copy sharing
- Constructors: `from_string()`, `from_arc_bytes()`, `from_arc_slice()` (compatibility shim for legacy `Arc<[u8]>`, incurs one extra copy)
- Data access: `as_bytes()` — unified `&[u8]` accessor, zero-copy
- Conversion: `to_bytes()` — convert to `Bytes` (copies when necessary); `into_bytes()` — consume self into `Bytes` (reuses buffer when possible)
- Utility methods: `len()`, `is_empty()`, `is_zero_copy()`
- Trait implementations: `Debug`, `Clone`, `Display` (lossy UTF-8 conversion for non-UTF-8 data)

### Dependencies

- Add `bytes = "1"` dependency

## [0.8.4]

### ⚠️ BREAKING CHANGES

- **`DataRecord` type change**: `Record<Field<Value>>` → `Record<FieldStorage>`, supports mixed Arc-shared and owned storage
- **`Value::Array` type change**: `Vec<Field<Value>>` → `Vec<FieldStorage>`, migrate with `.into_iter().collect()`
- **`ObjectValue` internal storage change**: `BTreeMap<SmolStr, Field<Value>>` → `BTreeMap<SmolStr, FieldStorage>`, auto-converts via `From` trait
- **`FieldStorage` restructured**: enum → struct + `ValueStorage` enum; `FieldStorage::Owned(field)` / `FieldStorage::Shared(arc)` no longer valid, use `from_owned()` / `from_shared()` instead
- **`FieldStorage::from_shared()` signature change**: `from_shared(Field<Value>)` → `from_shared(Arc<Field<Value>>)`, caller controls Arc creation
- **`get_field(usize)` renamed to `field_at(usize)`**, `get_field` now looks up by name
- **`DataRecord::get_field()` return type change**: `Option<&Field<Value>>` → `Option<FieldRef<'_>>`, provides cur_name-aware zero-copy access
- **License**: Elastic-2.0 → Apache-2.0

### Added

#### FieldStorage — Mixed Ownership Storage

- `FieldStorage` struct — field storage supporting zero-copy sharing and exclusive ownership
  - `ValueStorage::Shared(Arc<Field<Value>>)` — Arc-shared, clone only increments ref count (~5ns)
  - `ValueStorage::Owned(Field<Value>)` — exclusive ownership
- Core methods: `from_shared()`, `from_owned()`, `as_field()`, `as_field_mut()` (clone-on-write), `into_owned()`, `is_shared()`, `is_owned()`, `shared_count()`
- Trait implementations: `RecordItem`, `RecordItemFactory`, `LevelFormatAble`, `Display`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`

#### Zero-Copy set_name

- `set_name(impl Into<FNameStr>)` — only modifies the `cur_name` overlay, does not clone the underlying `DataField`
- `get_name()` — priority: `cur_name` > `field.name`
- `get_value()` / `get_meta()` — convenience accessors
- `cur_name` type is `FNameStr` (SmolStr), inline storage for ≤22 bytes, zero heap allocation

#### FieldRef — Zero-Copy Field Reference

- `FieldRef<'a>` — 8-byte zero-allocation wrapper providing a cur_name-aware consistent view
  - `get_name()` / `get_meta()` / `get_value()` — zero-copy access, returns `'a` lifetime references
  - `to_owned()` — lazily clones and applies cur_name
  - `has_name_override()` / `is_shared()` / `shared_count()`
  - Traits: `Debug`, `Display`, `PartialEq<Field<Value>>`, `PartialEq<FieldRef>`, `Copy`, `Clone`
- `FieldStorage::field_ref()` — obtain a `FieldRef`
- `DataRecord::get_field()` — returns `Option<FieldRef<'_>>` (cur_name-aware)
- `DataRecord::get_field_owned()` — get an owned field with cur_name applied
- `DataRecord::field_refs()` — cur_name-aware field iterator

#### Record API

- `push_shared()`, `push_owned()` — add fields by storage type
- `field_at(usize)` — access by index
- `get_field(name)`, `get_field_mut(name)` — lookup by name
- `fields()`, `fields_mut()` — field iterators (Shared variant uses clone-on-write)
- `storage_stats()` — report Shared/Owned counts
- `into_owned_record()` — convert to `Record<Field<Value>>`
- `append(impl Into<T>)`, `push()` — generic append
- `len()`, `is_empty()` — utility methods

#### Type Conversions

- `From<Field<Value>>`, `From<Arc<Field<Value>>>` → `FieldStorage`
- `From<Vec<DataField>>`, `From<DataField>` → `DataRecord`
- `FromIterator<Field<Value>>` → `Vec<FieldStorage>`
- `From<BTreeMap<SmolStr, DataField>>` → `ObjectValue`
- `ObjectValue::insert()` accepts `impl Into<FieldStorage>`
- `Field` type re-exported from `wp_model_core::model`

### Performance Improvements

- Shared field clone: 50–500ns → ~5ns (Arc ref count increment)
- Zero-copy rename: `set_name()` does not trigger `Arc<DataField>` clone
- Multi-stage pipelines: 2–3x performance improvement, multiple stages share the same Arc with independent naming
- Memory footprint: ~70% reduction by avoiding redundant clones of large field values

### Migration Guide

```rust
// Basic usage — no changes required
let mut record = DataRecord::default();

// Mixed storage + zero-copy rename
let field = Arc::new(Field::new(DataType::Chars, "HOST", Value::from("192.168.1.1")));
record.push_shared(Arc::clone(&field));

let mut storage = FieldStorage::from_shared(Arc::clone(&field));
storage.set_name("server_ip");  // zero-copy!

// Dynamic fields
record.push_owned(Field::new(DataType::Digit, "count", Value::from(42)));
record.append(FieldStorage::from_digit("age", 30));

// Value::Array migration
let arr = Value::Array(vec![field1, field2].into_iter().collect());

// ObjectValue
obj.insert("key".into(), field.into());

// Convert to fully owned
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

[Unreleased]: https://github.com/wp-labs/wp-model-core/compare/v0.8.6...HEAD
[0.8.6]: https://github.com/wp-labs/wp-model-core/compare/v0.8.5...v0.8.6
[0.8.5]: https://github.com/wp-labs/wp-model-core/compare/v0.8.4...v0.8.5
[0.8.4]: https://github.com/wp-labs/wp-model-core/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/wp-labs/wp-model-core/compare/v0.7.2...v0.8.3
[0.7.2]: https://github.com/wp-labs/wp-model-core/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/wp-labs/wp-model-core/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/wp-labs/wp-model-core/releases/tag/v0.7.0
