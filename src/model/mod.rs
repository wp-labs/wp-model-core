use crate::model::data::record::Record;
pub use crate::model::types::value::Value;
use data::field::Field;
use std::sync::Arc;
pub mod error;
pub mod fmt_def;
pub mod format;
mod macros;

//pub mod array;
// compare impls moved to orion_exp adapters
//mod conv;
pub mod data;
pub mod types;
// conditions impls moved out; core remains pure types + format

pub use types::meta::{DataType, MetaErr};
pub use types::value::{DateTimeValue, DomainT, EmailT, IdCardT, Maker, MobilePhoneT, UrlValue};
pub use types::value::{DigitValue, FloatValue, HexT, IgnoreT, IpNetValue};

/// 字段名称类型
/// 当前实现：SmolStr（小字符串优化，≤22字节内联存储）
/// 未来可以根据性能测试调整为其他实现
pub type FNameStr = smol_str::SmolStr;

/// 字段值字符串类型
/// 当前实现：SmolStr（小字符串优化，≤22字节内联存储）
/// 用于 Value::Chars，提供高效的短字符串存储
pub type FValueStr = smol_str::SmolStr;

pub type DataField = Field<Value>;
//VBean<Value>
pub type DataRecord = Record<DataField>;
//pub type SharedField = Field<Arc<Value>>;
pub type SharedRecord = Record<Arc<DataField>>;
//oml proc using

//pub use conv::to_shared_field_vec;
//pub use conv::to_value_field_vec;
