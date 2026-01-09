# wp-model-core Developer Guide

`wp-model-core` exposes the core data model used across warp-parse: fields, records, value types, and formatting helpers. This document walks through the major modules and common usage patterns.

## 1. Module Overview

```
wp-model-core
├── model
│   ├── data        // Field / Record / maker helpers
│   ├── types       // DataType & Value system
│   ├── fmt_def     // Output format definitions
│   ├── format      // LevelFormatAble helpers
│   └── macros      // value_match!, format_value!, ...
└── traits.rs       // Shared traits such as AsValueRef
```

### 1.1 Field & Record

- `Field<T>` wraps the name, `DataType` metadata, and the actual value `T`; the name itself is stored as an `ArcStr` so multiple sinks can share it at fanout time.
- `Record<T>` stores a vector of fields. Common aliases:
  - `DataField = Field<Value>`
  - `DataRecord = Record<DataField>`
  - `SharedRecord = Record<Arc<DataField>>`

## 2. Field API

```rust
use wp_model_core::model::{DataField, DataType, Value};

let f = DataField::from_chars("user", "alice");
assert_eq!(f.get_name(), "user");
assert_eq!(f.get_meta(), &DataType::Chars);
assert_eq!(f.get_value(), &Value::Chars("alice".into()));

let shared = DataField::from_shared_chars("user", arcstr::ArcStr::from("alice"));
assert_eq!(shared.get_chars(), Some("alice"));
```

Factory methods live in `model/data/maker.rs`:
- `from_bool`, `from_chars`, `from_shared_chars`, `from_digit`, `from_float` for primitives.
- `from_ip`, `from_domain`, `from_url`, etc. for semantic types.
- `from_arr`, `from_obj` for composite values.

## 3. Record API

```rust
use wp_model_core::model::{DataField, DataRecord};

let mut record = DataRecord::default();
record.append(DataField::from_digit("age", 18));
record.append(DataField::from_bool("active", true));
record.set_id(42); // Inserts wp_event_id (u64 → i64; silently skips on overflow)

if let Some(field) = record.field("age") {
    println!("age = {:?}", field.get_value());
}
```

Notes:
- `field()` / `get_value()` return the first field with the requested name.
- `get_value_mut()` yields a mutable reference (only valid for owned `Field<Value>`; panics for Rc/Arc-backed fields).
- `remove_field()` deletes the first field that matches the name.

## 4. Value System

`Value` covers primitive, semantic, and composite data (see `model/types/value`). Example:

```rust
use wp_model_core::model::Value;

let v = Value::Digit(123);
assert_eq!(v.tag(), "Digit");
assert!(!v.is_empty());
```

Helpful traits:
- `AsValueRef` allows `Field<Rc<Value>>` and similar wrappers to expose read-only references.
- `Maker<T>` provides the generic constructors used by `Field<T>` factory methods.

## 5. DataType Metadata

`DataType` describes the logical type of a field and supports friendly aliases:

```rust
use wp_model_core::model::DataType;

assert_eq!(DataType::from("http_request").unwrap(), DataType::HttpRequest);
assert_eq!(DataType::from("array/json").unwrap(), DataType::Array("json".into()));
```

Tip: `array` must include the subtype (e.g., `array/json`). Otherwise `MetaErr::UnSupport` is returned.

## 6. Utilities

- **`TagSet` (Deprecated)**: ⚠️ **This type is deprecated. Please use `Tags` from `wp-connector-api::runtime::source::types` instead.**

  `TagSet` was a lightweight key/value collection backed by a sorted `SmallVec` (up to 16 entries stored on the stack). It has been replaced by `Tags` which provides the same functionality with better API design:

  ```rust
  // Old (deprecated):
  use wp_model_core::model::TagSet;
  let mut tags = TagSet::default();
  tags.append("env", "prod");
  tags.set_tag("stage", "sink".into());

  // New (recommended):
  use wp_connector_api::runtime::source::types::Tags;
  let mut tags = Tags::new();
  tags.set("env", "prod");
  tags.set("stage", "sink");
  ```

- `LevelFormatAble` / `format_value!`: helpers for pretty-printing nested structures.
- `OutFmt` / `TextFmt`: describe serialization formats (JSON, CSV, RAW, ...).

## 7. Macros

- `value_match!`: dispatch by `Value` variant using `$crate::model::Value` to avoid additional imports.
- `format_value!`: reuse `Value` variant logic when implementing custom formatters.

## 8. Best Practices

1. **Avoid panics**: prefer `OrDefault` / `Result` flows unless failure is truly fatal.
2. **Mutable access**: when mutations are required, hold an owned `Value` instead of wrapping it in Rc/Arc. `Value::is_empty()` now inspects semantic types without cloning, so calling it on hot paths is cheap.
3. **Keep types consistent**: align field names with their `DataType` to simplify downstream formatting and conversions.

## 9. References

- `model/data/field.rs`, `record.rs`
- `model/types/meta.rs`, `value/*`
- `model/fmt_def.rs`, `format.rs`

See other warp-parse crates for more involved usage examples.
