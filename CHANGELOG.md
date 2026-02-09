# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.3]

### ⚠️ BREAKING CHANGES

- **`DataRecord` 类型变更**: `Record<Field<Value>>` → `Record<FieldStorage>`，支持 Arc 共享与独占混合存储
- **`Value::Array` 类型变更**: `Vec<Field<Value>>` → `Vec<FieldStorage>`，用 `.into_iter().collect()` 迁移
- **`ObjectValue` 内部存储变更**: `BTreeMap<SmolStr, Field<Value>>` → `BTreeMap<SmolStr, FieldStorage>`，通过 `From` trait 自动转换
- **`FieldStorage` 重构**: enum → struct + `ValueStorage` enum，不再能用 `FieldStorage::Owned(field)` / `FieldStorage::Shared(arc)` 构造，改用 `from_owned()` / `from_shared()`
- **`FieldStorage::from_shared()` 签名变更**: `from_shared(Field<Value>)` → `from_shared(Arc<Field<Value>>)`，调用方控制 Arc 创建
- **`get_field(usize)` 重命名为 `field_at(usize)`**，`get_field` 改为按名称查找
- **License**: Elastic-2.0 → Apache-2.0

### Added

#### FieldStorage 混合存储

- `FieldStorage` struct — 支持零拷贝共享与独占所有权的字段存储
  - `ValueStorage::Shared(Arc<Field<Value>>)` — Arc 共享，clone 仅增引用计数（~5ns）
  - `ValueStorage::Owned(Field<Value>)` — 独占所有权
- 核心方法: `from_shared()`, `from_owned()`, `as_field()`, `as_field_mut()`（clone-on-write）, `into_owned()`, `is_shared()`, `is_owned()`, `shared_count()`
- Trait 实现: `RecordItem`, `RecordItemFactory`, `LevelFormatAble`, `Display`, `PartialEq`, `Eq`, `Serialize`, `Deserialize`

#### 零拷贝 set_name

- `set_name(impl Into<FNameStr>)` — 仅修改 `cur_name` 覆盖层，不 clone 底层 `DataField`
- `get_name()` — 优先级: `cur_name` > `field.name`
- `get_value()` / `get_meta()` — 便捷访问器
- `cur_name` 类型为 `FNameStr`（SmolStr），≤22 字节内联存储，零堆分配

#### Record API

- `push_shared()`, `push_owned()` — 按存储类型添加字段
- `field_at(usize)` — 按索引访问
- `get_field(name)`, `get_field_mut(name)` — 按名称查找
- `fields()`, `fields_mut()` — 字段迭代器（Shared 变体 clone-on-write）
- `storage_stats()` — 统计 Shared/Owned 数量
- `into_owned_record()` — 转换为 `Record<Field<Value>>`
- `append(impl Into<T>)`, `push()` — 泛型追加
- `len()`, `is_empty()` — 实用方法

#### 类型转换

- `From<Field<Value>>`, `From<Arc<Field<Value>>>` → `FieldStorage`
- `From<Vec<DataField>>`, `From<DataField>` → `DataRecord`
- `FromIterator<Field<Value>>` → `Vec<FieldStorage>`
- `From<BTreeMap<SmolStr, DataField>>` → `ObjectValue`
- `ObjectValue::insert()` 接受 `impl Into<FieldStorage>`
- `Field` 类型从 `wp_model_core::model` 公开导出

### Performance Improvements

- Shared 字段 clone: 50-500ns → ~5ns（Arc 引用计数递增）
- 零拷贝重命名: `set_name()` 不触发 `Arc<DataField>` clone
- 多阶段管道: 2-3x 性能提升，多阶段共享同一 Arc 各自独立命名
- 内存占用: ~70% 节省，避免重复 clone 大字段值

### Migration Guide

```rust
// 基本用法无需修改
let mut record = DataRecord::default();

// 混合存储 + 零拷贝重命名
let field = Arc::new(Field::new(DataType::Chars, "HOST", Value::from("192.168.1.1")));
record.push_shared(Arc::clone(&field));

let mut storage = FieldStorage::from_shared(Arc::clone(&field));
storage.set_name("server_ip");  // 零拷贝！

// 动态字段
record.push_owned(Field::new(DataType::Digit, "count", Value::from(42)));
record.append(FieldStorage::from_digit("age", 30));

// Value::Array 迁移
let arr = Value::Array(vec![field1, field2].into_iter().collect());

// ObjectValue
obj.insert("key".into(), field.into());

// 转换为全量 owned
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

[Unreleased]: https://github.com/wp-labs/wp-model-core/compare/v0.8.3...HEAD
[0.8.3]: https://github.com/wp-labs/wp-model-core/compare/v0.7.2...v0.8.3
[0.7.2]: https://github.com/wp-labs/wp-model-core/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/wp-labs/wp-model-core/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/wp-labs/wp-model-core/releases/tag/v0.7.0
