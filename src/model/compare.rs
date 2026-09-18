//! **NOT COMPILED**：本文件未挂进模块树（`model/mod.rs` 无 `mod compare;`，比较实现已迁到
//! `orion_exp` 适配器），且 `orion_exp` 已不是本 crate 的依赖、`is_support` 的 `match` 也缺
//! `Value::BigUint` 分支 —— 自 0.9.0 起就已失效。保留仅供迁移参考；**删改前请先确认是否
//! 还需要它**（本次 `Value::Digit` → `Int` 的改名只是顺手保持一致，不构成它已被启用）。

use crate::model::Value;
use crate::{model::data::field::Field, traits::AsValueRef};
use orion_exp::{CmpOperator, ValueComparator};

impl<T> ValueComparator for Field<T>
where
    T: AsValueRef<Value>,
{
    fn is_support(&self, op: CmpOperator) -> bool {
        match self.get_value() {
            Value::Bool(v) => v.is_support(op),
            Value::Chars(v) => v.to_string().is_support(op),
            Value::Symbol(v) => v.to_string().is_support(op),
            Value::Time(v) => v.is_support(op),
            Value::Int(v) => v.is_support(op),
            Value::Hex(v) => v.is_support(op),
            Value::Float(v) => v.is_support(op),
            Value::IpNet(v) => v.is_support(op),
            Value::IpAddr(v) => v.is_support(op),
            Value::Null => true,
            Value::Ignore(_) => true,
            Value::Obj(_v) => true,
            Value::Array(_v) => true,
            Value::Domain(_) => true,
            Value::Email(_) => true,
            Value::Url(_) => true,
            Value::IdCard(_) => true,
            Value::MobilePhone(_) => true,
        }
    }

    fn compare_with(&self, other: &Self, op: &CmpOperator) -> bool {
        // Special case: RHS is Ignore (sentinel for `isset($var)`) → true when LHS exists
        if matches!(other.get_value(), Value::Ignore(_)) {
            return true;
        }
        if std::mem::discriminant(self.get_value()) != std::mem::discriminant(other.get_value()) {
            return false;
        }
        match (self.get_value(), other.get_value()) {
            (Value::Chars(v1), Value::Chars(v2)) => v1.to_string().compare_with(&v2.to_string(), op),
            (Value::Symbol(v1), Value::Symbol(v2)) => v1.to_string().compare_with(&v2.to_string(), op),
            (Value::Time(v1), Value::Time(v2)) => v1.compare_with(v2, op),
            (Value::Bool(v1), Value::Bool(v2)) => v1.compare_with(v2, op),
            (Value::Int(v1), Value::Int(v2)) => v1.compare_with(v2, op),
            (Value::Hex(v1), Value::Hex(v2)) => v1.compare_with(v2, op),
            (Value::Float(v1), Value::Float(v2)) => v1.compare_with(v2, op),
            (Value::IpNet(v1), Value::IpNet(v2)) => v1.compare_with(v2, op),
            (Value::IpAddr(v1), Value::IpAddr(v2)) => v1.compare_with(v2, op),
            (Value::Obj(_v1), Value::Obj(_v2)) => todo!(),
            (Value::Array(_v1), Value::Array(_v2)) => todo!(),
            (Value::Ignore(_), Value::Ignore(_)) => true,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
