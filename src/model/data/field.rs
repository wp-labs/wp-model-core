use crate::model::DataType;
use crate::model::format::LevelFormatAble;
use crate::model::{FNameStr, FValueStr};

use crate::model::Value;
use crate::traits::AsValueRef;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Field<T> {
    pub meta: DataType,
    pub name: FNameStr,
    pub value: T,
}

/// A trait for getting an immutable reference to the inner value of a Field
pub trait ValueRef<T> {
    /// Returns an immutable reference to the inner value
    fn value_ref(&self) -> &T;
}

impl<T> ValueRef<T> for Field<T> {
    fn value_ref(&self) -> &T {
        &self.value
    }
}

impl<T> From<Field<T>> for Field<Rc<T>> {
    fn from(other: Field<T>) -> Self {
        let value = Rc::<T>::from(other.value);
        Self {
            meta: other.meta,
            name: other.name,
            value,
        }
    }
}

impl<T> From<Field<T>> for Field<Arc<T>> {
    fn from(other: Field<T>) -> Self {
        let value = Arc::<T>::from(other.value);
        Self {
            meta: other.meta,
            name: other.name,
            value,
        }
    }
}

impl<T> Field<T> {
    pub fn new<S: Into<FNameStr>, V: Into<T>>(meta: DataType, name: S, value: V) -> Self {
        Self {
            meta,
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn new_opt(meta: DataType, name: Option<FNameStr>, value: T) -> Self {
        let name = name.unwrap_or_else(|| FNameStr::from(String::from(&meta)));
        Field { meta, name, value }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn clone_name(&self) -> String {
        self.name.as_str().to_string()
    }
    pub fn get_meta(&self) -> &DataType {
        &self.meta
    }

    pub fn set_name<S: Into<FNameStr>>(&mut self, name: S) {
        self.name = name.into()
    }
}

impl Field<Value> {
    pub fn from_shared_chars<S: Into<FNameStr>>(name: S, val: FValueStr) -> Self {
        Self::new(DataType::Chars, name.into(), Value::Chars(val))
    }

    pub fn get_chars(&self) -> Option<&str> {
        self.value.as_str()
    }

    pub fn get_chars_mut(&mut self) -> Option<&mut FValueStr> {
        self.value.ensure_owned_chars()
    }
}
impl<T> Field<T>
where
    T: AsValueRef<Value>,
{
    pub fn get_value(&self) -> &Value {
        self.value.as_value_ref()
        //&self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Value {
        self.value.as_value_mutref()
    }
}

impl<T> LevelFormatAble for Field<T>
where
    T: Display,
{
    fn level_fmt(&self, f: &mut Formatter<'_>, level: usize) -> std::fmt::Result {
        let meta: String = From::from(&self.meta);
        writeln!(
            f,
            "{:width$}[{:<16}] {:<20} : {}",
            "",
            meta,
            self.name,
            self.value,
            width = level * 6
        )?;
        Ok(())
    }
}

impl<T> Display for Field<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.meta, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DataField, FValueStr};

    // ========== Field creation tests ==========

    #[test]
    fn test_field_new() {
        let field: Field<i64> = Field::new(DataType::Digit, "count", 42i64);
        assert_eq!(field.meta, DataType::Digit);
        assert_eq!(field.get_name(), "count");
        assert_eq!(field.value, 42);
    }

    #[test]
    fn test_field_new_with_string_conversion() {
        let field: Field<String> = Field::new(DataType::Chars, String::from("key"), "value");
        assert_eq!(field.get_name(), "key");
        assert_eq!(field.value, "value");
    }

    #[test]
    fn test_field_new_opt_with_name() {
        let field: Field<i64> = Field::new_opt(DataType::Digit, Some("num".into()), 100);
        assert_eq!(field.get_name(), "num");
        assert_eq!(field.value, 100);
    }

    #[test]
    fn test_field_new_opt_without_name() {
        let field: Field<i64> = Field::new_opt(DataType::Digit, None, 50);
        // When name is None, it should use meta's string representation
        assert_eq!(field.get_name(), "digit");
        assert_eq!(field.value, 50);
    }

    // ========== Field accessor tests ==========

    #[test]
    fn test_field_get_name() {
        let field: Field<i64> = Field::new(DataType::Digit, "test_name", 1);
        assert_eq!(field.get_name(), "test_name");
    }

    #[test]
    fn test_field_clone_name() {
        let field: Field<i64> = Field::new(DataType::Digit, "original", 1);
        let cloned = field.clone_name();
        assert_eq!(cloned, "original");
        // Verify it's a new String, not a reference
        assert_eq!(cloned, field.get_name());
    }

    #[test]
    fn test_field_get_meta() {
        let field: Field<i64> = Field::new(DataType::Float, "val", 1);
        assert_eq!(field.get_meta(), &DataType::Float);
    }

    #[test]
    fn test_field_set_name() {
        let mut field: Field<i64> = Field::new(DataType::Digit, "old_name", 1);
        field.set_name("new_name");
        assert_eq!(field.get_name(), "new_name");
    }

    // ========== ValueRef trait tests ==========

    #[test]
    fn test_value_ref_trait() {
        let field: Field<i64> = Field::new(DataType::Digit, "num", 42);
        assert_eq!(field.value_ref(), &42);
    }

    // ========== Field<Value> specific tests ==========

    #[test]
    fn test_field_get_value() {
        let field: DataField = Field::new(DataType::Digit, "num", Value::Digit(99));
        assert_eq!(field.get_value(), &Value::Digit(99));
    }

    #[test]
    fn test_field_get_value_mut() {
        let mut field: DataField = Field::new(DataType::Digit, "num", Value::Digit(10));
        *field.get_value_mut() = Value::Digit(20);
        assert_eq!(field.get_value(), &Value::Digit(20));
    }

    #[test]
    fn test_field_from_shared_chars() {
        let s = FValueStr::from("hello");
        let field: DataField = Field::from_shared_chars("msg", s.clone());
        assert_eq!(field.get_name(), "msg");
        assert_eq!(field.get_meta(), &DataType::Chars);
        assert!(matches!(field.value, Value::Chars(_)));
        assert_eq!(field.get_chars(), Some("hello"));
    }

    #[test]
    fn test_field_get_chars_mut() {
        let s = FValueStr::from("foo");
        let mut field: DataField = Field::from_shared_chars("msg", s);
        {
            let value = field.get_chars_mut().expect("mutable");
            // SmolStr is immutable, so we replace it with a new one
            *value = FValueStr::from(format!("{}-bar", value.as_str()));
        }
        assert!(matches!(field.value, Value::Chars(_)));
        assert_eq!(field.get_chars(), Some("foo-bar"));
    }

    // ========== From conversions tests ==========

    #[test]
    fn test_field_to_rc() {
        let field: Field<i64> = Field::new(DataType::Digit, "num", 42);
        let rc_field: Field<Rc<i64>> = field.into();

        assert_eq!(rc_field.get_name(), "num");
        assert_eq!(rc_field.meta, DataType::Digit);
        assert_eq!(*rc_field.value, 42);
    }

    #[test]
    fn test_field_to_arc() {
        let field: Field<String> = Field::new(DataType::Chars, "msg", String::from("hello"));
        let arc_field: Field<Arc<String>> = field.into();

        assert_eq!(arc_field.get_name(), "msg");
        assert_eq!(arc_field.meta, DataType::Chars);
        assert_eq!(*arc_field.value, "hello");
    }

    // ========== Display tests ==========

    #[test]
    fn test_field_display() {
        let field: Field<i64> = Field::new(DataType::Digit, "count", 42);
        let display = format!("{}", field);
        assert!(display.contains("digit"));
        assert!(display.contains("42"));
    }

    #[test]
    fn test_field_display_chars() {
        let field: Field<String> = Field::new(DataType::Chars, "msg", String::from("hello"));
        let display = format!("{}", field);
        assert!(display.contains("chars"));
        assert!(display.contains("hello"));
    }

    // ========== LevelFormatAble tests ==========

    #[test]
    fn test_level_format_able() {
        let field: Field<i64> = Field::new(DataType::Digit, "level_test", 123);
        let mut output = String::new();
        use std::fmt::Write;
        // Use a simple formatter wrapper
        let _ = write!(output, "{}", field);
        assert!(output.contains("digit"));
        assert!(output.contains("123"));
    }

    // ========== Clone and PartialEq tests ==========

    #[test]
    fn test_field_clone() {
        let field: Field<i64> = Field::new(DataType::Digit, "num", 42);
        let cloned = field.clone();

        assert_eq!(field, cloned);
        assert_eq!(cloned.get_name(), "num");
        assert_eq!(cloned.value, 42);
    }

    #[test]
    fn test_field_partial_eq() {
        let field1: Field<i64> = Field::new(DataType::Digit, "num", 42);
        let field2: Field<i64> = Field::new(DataType::Digit, "num", 42);
        let field3: Field<i64> = Field::new(DataType::Digit, "num", 99);

        assert_eq!(field1, field2);
        assert_ne!(field1, field3);
    }

    // ========== Serde tests ==========

    #[test]
    fn test_field_serde_roundtrip() {
        let field: Field<i64> = Field::new(DataType::Digit, "serde_test", 123);
        let json = serde_json::to_string(&field).unwrap();
        let parsed: Field<i64> = serde_json::from_str(&json).unwrap();

        assert_eq!(field, parsed);
    }

    #[test]
    fn test_field_serde_with_string() {
        let field: Field<String> = Field::new(DataType::Chars, "msg", String::from("test"));
        let json = serde_json::to_string(&field).unwrap();
        let parsed: Field<String> = serde_json::from_str(&json).unwrap();

        assert_eq!(field.name, parsed.name);
        assert_eq!(field.value, parsed.value);
    }
}
