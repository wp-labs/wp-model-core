// FieldRef: Zero-copy wrapper for cur_name-aware field access

use crate::model::Field;
use crate::model::data::storage::FieldStorage;
use crate::model::{DataType, Value};

/// Zero-copy field reference with cur_name overlay applied.
///
/// `FieldRef` provides a unified view of `FieldStorage` that respects
/// the `cur_name` override without cloning the underlying field.
///
/// # Performance
///
/// - **Zero allocation**: Only holds a reference to `FieldStorage` (8 bytes)
/// - **Zero-copy access**: `get_name()`, `get_value()` etc. don't clone
/// - **Lazy owned conversion**: Cloning happens only when calling `to_owned()`
///
/// # Examples
///
/// ```ignore
/// use std::sync::Arc;
/// use wp_model_core::model::{Field, FieldStorage, Value, DataType};
///
/// let field = Arc::new(Field::new(DataType::Chars, "original", Value::from("data")));
/// let mut storage = FieldStorage::from_shared(field);
/// storage.set_name("renamed");
///
/// let field_ref = storage.field_ref();
/// assert_eq!(field_ref.get_name(), "renamed");
/// assert_eq!(field_ref.get_value(), &Value::from("data"));
/// ```
#[derive(Clone, Copy)]
pub struct FieldRef<'a> {
    pub(crate) storage: &'a FieldStorage,
}

impl<'a> FieldRef<'a> {
    /// Get effective field name (cur_name takes priority).
    #[inline]
    pub fn get_name(&self) -> &'a str {
        self.storage.get_name()
    }

    /// Get field metadata type.
    #[inline]
    pub fn get_meta(&self) -> &'a DataType {
        self.storage.as_field().get_meta()
    }

    /// Get field value reference.
    #[inline]
    pub fn get_value(&self) -> &'a Value {
        self.storage.as_field().get_value()
    }

    /// Convert to owned field with cur_name applied.
    ///
    /// Clones the underlying field and applies the `cur_name` override if present.
    pub fn to_owned(self) -> Field<Value> {
        self.storage.clone().into_owned()
    }

    /// Check if this field is shared (Arc-backed).
    #[inline]
    pub fn is_shared(&self) -> bool {
        self.storage.is_shared()
    }

    /// Check if cur_name override is active.
    #[inline]
    pub fn has_name_override(&self) -> bool {
        self.storage.cur_name.is_some()
    }

    /// Get Arc reference count for debugging.
    ///
    /// Returns `Some(count)` for `Shared` variant, `None` for `Owned`.
    #[inline]
    pub fn shared_count(&self) -> Option<usize> {
        self.storage.shared_count()
    }
}

// ==================== Trait Implementations ====================

impl<'a> std::fmt::Debug for FieldRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldRef")
            .field("name", &self.get_name())
            .field("meta", &self.get_meta())
            .field("value", &self.get_value())
            .field("is_shared", &self.is_shared())
            .field("has_override", &self.has_name_override())
            .finish()
    }
}

impl<'a> std::fmt::Display for FieldRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:?}", self.get_name(), self.get_value())
    }
}

impl<'a> PartialEq<Field<Value>> for FieldRef<'a> {
    fn eq(&self, other: &Field<Value>) -> bool {
        self.get_name() == other.get_name()
            && self.get_meta() == other.get_meta()
            && self.get_value() == other.get_value()
    }
}

impl<'a, 'b> PartialEq<FieldRef<'b>> for FieldRef<'a> {
    fn eq(&self, other: &FieldRef<'b>) -> bool {
        self.get_name() == other.get_name()
            && self.get_meta() == other.get_meta()
            && self.get_value() == other.get_value()
    }
}

impl<'a> PartialEq<FieldRef<'a>> for Field<Value> {
    fn eq(&self, other: &FieldRef<'a>) -> bool {
        other == self
    }
}

// ==================== Tests ====================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_field_ref_respects_cur_name() {
        let field = Field::new(DataType::Chars, "old_name", Value::from("value"));
        let mut storage = FieldStorage::from_owned(field);
        storage.set_name("new_name");

        let field_ref = storage.field_ref();

        assert_eq!(field_ref.get_name(), "new_name");
        assert_eq!(field_ref.get_value(), &Value::from("value"));

        // Underlying field unchanged
        assert_eq!(storage.as_field().get_name(), "old_name");
    }

    #[test]
    fn test_field_ref_no_cur_name() {
        let field = Field::new(DataType::Digit, "count", Value::from(42));
        let storage = FieldStorage::from_owned(field);

        let field_ref = storage.field_ref();

        assert_eq!(field_ref.get_name(), "count");
        assert_eq!(field_ref.get_value(), &Value::from(42));
        assert!(!field_ref.has_name_override());
    }

    #[test]
    fn test_field_ref_shared_zero_copy() {
        let field = Arc::new(Field::new(DataType::Chars, "original", Value::from("data")));
        let arc_count_before = Arc::strong_count(&field);

        let mut storage = FieldStorage::from_shared(field.clone());
        storage.set_name("renamed");

        // field_ref() doesn't clone Arc
        let field_ref = storage.field_ref();
        assert_eq!(Arc::strong_count(&field), arc_count_before + 1);

        assert_eq!(field_ref.get_name(), "renamed");
        assert_eq!(field_ref.get_value(), &Value::from("data"));
        assert!(field_ref.is_shared());
        assert!(field_ref.has_name_override());
    }

    #[test]
    fn test_field_ref_to_owned() {
        let mut storage =
            FieldStorage::from_owned(Field::new(DataType::Digit, "count", Value::from(42)));
        storage.set_name("total");

        let field_ref = storage.field_ref();
        let owned = field_ref.to_owned();

        assert_eq!(owned.get_name(), "total");
        assert_eq!(owned.get_value(), &Value::from(42));
    }

    #[test]
    fn test_field_ref_shared_count() {
        let field = Arc::new(Field::new(DataType::Chars, "name", Value::from("test")));
        let storage1 = FieldStorage::from_shared(field.clone());
        let storage2 = FieldStorage::from_shared(field.clone());

        assert_eq!(storage1.field_ref().shared_count(), Some(3));
        assert_eq!(storage2.field_ref().shared_count(), Some(3));
    }

    #[test]
    fn test_field_ref_owned_no_shared_count() {
        let storage =
            FieldStorage::from_owned(Field::new(DataType::Chars, "name", Value::from("test")));

        assert_eq!(storage.field_ref().shared_count(), None);
    }

    #[test]
    fn test_field_ref_equality() {
        let field1 = Field::new(DataType::Chars, "name", Value::from("Alice"));
        let storage1 = FieldStorage::from_owned(field1.clone());

        let field2 = Field::new(DataType::Chars, "name", Value::from("Alice"));
        let storage2 = FieldStorage::from_owned(field2.clone());

        assert_eq!(storage1.field_ref(), storage2.field_ref());
        assert_eq!(storage1.field_ref(), field1);
        assert_eq!(field2, storage2.field_ref());
    }

    #[test]
    fn test_field_ref_inequality_with_cur_name() {
        let mut storage1 =
            FieldStorage::from_owned(Field::new(DataType::Chars, "name", Value::from("Alice")));
        storage1.set_name("renamed");

        let storage2 =
            FieldStorage::from_owned(Field::new(DataType::Chars, "name", Value::from("Alice")));

        assert_ne!(storage1.field_ref(), storage2.field_ref());
    }

    #[test]
    fn test_field_ref_debug() {
        let mut storage =
            FieldStorage::from_owned(Field::new(DataType::Chars, "test", Value::from("value")));
        storage.set_name("renamed");

        let debug = format!("{:?}", storage.field_ref());
        assert!(debug.contains("renamed"));
        assert!(debug.contains("Chars"));
        assert!(debug.contains("has_override: true"));
    }

    #[test]
    fn test_field_ref_display() {
        let storage =
            FieldStorage::from_owned(Field::new(DataType::Digit, "count", Value::from(42)));

        let display = format!("{}", storage.field_ref());
        assert_eq!(display, "count=Digit(42)");
    }

    #[test]
    fn test_field_ref_copy() {
        let storage =
            FieldStorage::from_owned(Field::new(DataType::Chars, "name", Value::from("test")));

        let ref1 = storage.field_ref();
        let ref2 = ref1; // Copy

        assert_eq!(ref1.get_name(), ref2.get_name());
    }
}
