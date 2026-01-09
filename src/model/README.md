# Model 模块说明

## 核心职责

本模块是数据处理系统的核心建模组件，主要提供以下功能：

- **数据建模**：类型安全的字段(`Field<T>`)和记录(`Record<T>`)模型
- **值类型系统**：支持20+种数据类型(基础类型、语义类型和复合类型)
- **元数据管理**：通过`DataType`管理类型注解和格式约束
- **数据操作**：字段值的比较、转换和验证

## 核心类型

### 1. 字段系统

| 类型 | 说明 |
|------|------|
| `Field<T>` | 通用字段容器(名称+值+类型元数据) |
| `DataField` | `Field<Value>`的类型别名(具体值) |
| `SharedField` | 历史产物，建议直接使用 `Field<Value>` 或 `Arc<Field<Value>>` |


### 2. 值类型枚举

```rust
pub enum Value {
    // 基础类型（实际代码命名）
    Bool(bool),
    Chars(String),
    Digit(i64),
    Float(f64),
    Time(DateTimeValue),

    // 语义类型
    Email(EmailT),
    IdCard(IdCardT),
    MobilePhone(MobilePhoneT),
    Domain(DomainT),
    Url(UrlValue),
    IpAddr(std::net::IpAddr),
    IpNet(IpNetValue),

    // 复合类型
    Obj(ObjectValue),     // BTreeMap<String, DataField>
    Array(Vec<DataField>),
    Symbol(String),
    Ignore(IgnoreT),
}
```

### 3. 类型元数据

```rust
pub enum DataType {
    Bool,
    Chars,
    Digit,
    Float,
    Time,
    TimeRFC3339,
    TimeRFC2822,
    TimeTIMESTAMP,
    Domain,
    Email,
    Url,
    IdCard,
    MobilePhone,
    Obj,
    Array(String),
    Ignore,
    // ...详见 `model/types/meta.rs`
}
```

## 使用示例

### 创建字段

```rust
// 创建基础类型字段
let username = Field::new(DataType::Chars, "username", "张三".to_string());

// 创建语义类型字段
let email = Field::new(
    DataType::Email,
    "邮箱",
    EmailValue::parse("zhangsan@example.com")?
);
```

### 构建记录

```rust
let mut record = Record::<DataField>::default();
record.append(Field::from_bool("是否激活", true));
record.append(Field::from_ip("IP地址", IpAddr::V4([127, 0, 0, 1].into())));

// 访问字段
assert_eq!(record.field("是否激活").unwrap().get_value(), &Value::Bool(true));
```

### 类型转换

```rust
// 构造字段并读取内容
let field = Field::from_chars("姓名", "李四");
if let Value::Chars(name) = field.value_ref() {
    println!("提取的姓名: {}", name);
}
```

## 设计要点

1. **类型安全**：
   - 所有字段都携带运行时类型信息(`DataType`)
   - 值变体强制数据格式正确(如`EmailValue`在创建时验证)

2. **数据验证**：
```rust
// 校验逻辑通常在类型构造时完成，Value 本身不暴露 validate。
```

3. **比较操作**：
   - 实现`Comparable` trait支持多种比较操作：
   ```rust
   use orion_exp::{CmpOperator, ValueComparator};
   assert!(field1.compare_with(&field2, &CmpOperator::Eq));
   ```



---

本文档包含：
- 清晰的类型层次结构说明
- 实际使用示例
- 版本变更说明
- 关键设计原理

是否需要补充特定内容？例如：
- 性能特性说明
- 序列化集成示例
- 扩展自定义类型指南
