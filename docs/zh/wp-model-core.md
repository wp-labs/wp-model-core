# wp-model-core 开发指南

`wp-model-core` 提供 warp-pase 的核心数据模型：字段、记录、值类型及相关工具（格式化等）。本指南概述主要模块并给出常见用例。

## 1. 数据结构总览

```
wp-model-core
├── model
│   ├── data        // Field / Record / Maker
│   ├── types       // DataType + Value 体系
│   ├── fmt_def     // 输出格式定义
│   ├── format      // LevelFormatAble 等格式化工具
│   └── macros      // value_match! 等辅助宏
└── traits.rs       // AsValueRef 等通用 trait
```

### 1.1 Field 与 Record

- `Field<T>`：封装字段名、类型元信息 `DataType` 与值 `T`；字段名内部使用 `ArcStr`，以便 fanout/多 sink 场景下共享。
- `Record<T>`：字段集合（Vec）。常用别名：
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

创建方法（位于 `model/data/maker.rs`）：
- `from_bool/from_chars/from_shared_chars/from_digit/from_float` 等基础类型构造。
- `from_ip/from_domain/from_url` 等语义类型构造。
- `from_arr/from_obj` 支持复合类型。

## 3. Record API

```rust
use wp_model_core::model::{DataField, DataRecord};

let mut record = DataRecord::default();
record.append(DataField::from_digit("age", 18));
record.append(DataField::from_bool("active", true));
record.set_id(42); // 自动插入 wp_event_id（u64 转 i64，超界时忽略）

if let Some(field) = record.field("age") {
    println!("age = {:?}", field.get_value());
}
```

要点：
- `field()`/`get_value()` 返回首个同名字段。
- `get_value_mut()` 返回可变引用（仅适用于 `Field<Value>`，若值存储在 `Rc/Arc` 内则会 panic）。
- `remove_field()` 按名称删除第一项。

## 4. Value 体系

`Value` 枚举覆盖基础、语义、复合类型（详见 `model/types/value`）。常用路径：

```rust
use wp_model_core::model::Value;

let v = Value::Digit(123);
assert_eq!(v.tag(), "Digit");
assert!(!v.is_empty());
```

辅助 trait：
- `AsValueRef`：允许 `Field<Rc<Value>>` 等在只读模式下复用现有值。
- `Maker<T>`：用于 `Field<T>` 上的构造方法（泛型地创建内部值）。

## 5. DataType（元信息）

`DataType` 描述字段类型，并支持常用别名：

```rust
use wp_model_core::model::DataType;

assert_eq!(DataType::from("http_request").unwrap(), DataType::HttpRequest);
assert_eq!(DataType::from("array/json").unwrap(), DataType::Array("json".into()));
```

注意：`array` 必须带 `/子类型`，否则 `MetaErr::UnSupport`。

## 6. 辅助工具

- **`TagSet`（已废弃）**：⚠️ **此类型已被标记为废弃，请使用 `wp-connector-api::runtime::source::types` 中的 `Tags` 代替。**

  `TagSet` 是一个轻量级 KV 集合，内部使用排序 `SmallVec`（最多 16 项走栈内存）。现已被 `Tags` 替代，后者提供相同功能且 API 设计更优：

  ```rust
  // 旧方式（已废弃）：
  use wp_model_core::model::TagSet;
  let mut tags = TagSet::default();
  tags.append("env", "prod");
  tags.set_tag("stage", "sink".into());

  // 新方式（推荐）：
  use wp_connector_api::runtime::source::types::Tags;
  let mut tags = Tags::new();
  tags.set("env", "prod");
  tags.set("stage", "sink");
  ```

- `LevelFormatAble` / `format_value!`：层级格式化输出。
- `OutFmt`/`TextFmt`：描述记录序列化格式（JSON、CSV、RAW 等）。

## 7. 宏

- `value_match!`：按 `Value` 分派并调用闭包，已使用 `$crate::model::Value` 避免外部导入问题。
- `format_value!`：在实现自定义格式化时复用 Value 变体逻辑。

## 8. 最佳实践

1. **避免 panic**：使用 `OrDefault`/`Result` 处理错误，除非明确无法继续。
2. **Value 可变访问**：当需要修改值时，优先使用拥有型 `Value`，不要把 `Value` 放在共享智能指针里。`Value::is_empty()` 已针对语义类型做零拷贝判定，可放心在热路径调用。
3. **类型一致性**：字段命名要与 `DataType` 对应，便于后续格式化/转换。

## 9. 参考

- `model/data/field.rs` / `record.rs`
- `model/types/meta.rs` / `value/*`
- `model/fmt_def.rs` / `format.rs`

如需更深入示例，可参考 warp-pase 其他 crate 中的使用场景。
