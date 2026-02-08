use crate::model::DataType;
use crate::model::Value;
use crate::model::format::LevelFormatAble;
use crate::model::{FNameStr, FValueStr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::sync::Arc;

use super::field::Field;
use super::record::{RecordItem, RecordItemFactory};

/// Mixed storage for fields supporting both Arc (zero-copy) and owned (dynamic) variants.
///
/// This enum allows efficient handling of both static/constant fields (via `Shared`)
/// and dynamically computed fields (via `Owned`).
///
/// # Performance
/// - Cloning `Shared` variant: ~5ns (Arc reference count increment)
/// - Cloning `Owned` variant: 50-500ns (deep copy of Field<Value>)
/// - Accessing field via `as_field()`: ~0-1ns
///
/// # Examples
///
/// ```ignore
/// use std::sync::Arc;
/// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
///
/// // Create a shared field (for static/constant values)
/// let static_field = Field::new(DataType::Chars, "app_name", Value::from("myapp"));
/// let shared = FieldStorage::from_shared(static_field);
///
/// // Clone is cheap (only increments Arc reference count)
/// let shared2 = shared.clone();
///
/// // Create an owned field (for dynamic values)
/// let dynamic_field = Field::new(DataType::Digit, "counter", Value::from(42));
/// let owned = FieldStorage::from_owned(dynamic_field);
/// ```
#[derive(Clone, Debug)]
pub enum FieldStorage {
    /// Shared field with reference counting for zero-copy sharing.
    ///
    /// Used for static/constant fields that are referenced multiple times.
    /// Cloning this variant only increments the reference count (~5ns).
    Shared(Arc<Field<Value>>),

    /// Owned field from dynamic computation.
    ///
    /// Used for fields that are computed dynamically per record.
    /// Cloning this variant performs a deep copy of the field.
    Owned(Field<Value>),
}

impl FieldStorage {
    /// Get a reference to the underlying field (unified interface).
    ///
    /// # Performance
    /// - Shared variant: ~1ns (dereference)
    /// - Owned variant: 0ns (direct reference)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Field::new(DataType::Chars, "test", Value::from("value"));
    /// let storage = FieldStorage::from_owned(field);
    ///
    /// assert_eq!(storage.as_field().get_name(), "test");
    /// ```
    #[inline]
    pub fn as_field(&self) -> &Field<Value> {
        match self {
            FieldStorage::Shared(arc) => arc.as_ref(),
            FieldStorage::Owned(field) => field,
        }
    }

    /// Convert to owned Field<Value>.
    ///
    /// # Performance
    /// - Shared variant: ~50-500ns (clone the inner field if multiple references exist)
    /// - Owned variant: 0ns (move)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::sync::Arc;
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Field::new(DataType::Digit, "num", Value::from(42));
    /// let storage = FieldStorage::from_owned(field);
    /// let owned = storage.into_owned();
    ///
    /// assert_eq!(owned.get_name(), "num");
    /// ```
    pub fn into_owned(self) -> Field<Value> {
        match self {
            FieldStorage::Shared(arc) => {
                // Try to unwrap if this is the only reference, otherwise clone
                Arc::try_unwrap(arc).unwrap_or_else(|arc| (*arc).clone())
            }
            FieldStorage::Owned(field) => field,
        }
    }

    /// Create Shared variant from owned field.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Field::new(DataType::Chars, "name", Value::from("Alice"));
    /// let storage = FieldStorage::from_shared(field);
    ///
    /// assert!(storage.is_shared());
    /// ```
    pub fn from_shared(field: Field<Value>) -> Self {
        FieldStorage::Shared(Arc::new(field))
    }

    /// Create Owned variant from owned field.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Field::new(DataType::Digit, "count", Value::from(10));
    /// let storage = FieldStorage::from_owned(field);
    ///
    /// assert!(!storage.is_shared());
    /// ```
    pub fn from_owned(field: Field<Value>) -> Self {
        FieldStorage::Owned(field)
    }

    /// Check if this is a Shared variant.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Field::new(DataType::Chars, "test", Value::from("data"));
    /// let shared = FieldStorage::from_shared(field.clone());
    /// let owned = FieldStorage::from_owned(field);
    ///
    /// assert!(shared.is_shared());
    /// assert!(!owned.is_shared());
    /// ```
    #[inline]
    pub fn is_shared(&self) -> bool {
        matches!(self, FieldStorage::Shared(_))
    }

    /// Get Arc reference count (for debugging/diagnostics).
    ///
    /// Returns `Some(count)` for Shared variant, `None` for Owned variant.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Field::new(DataType::Digit, "x", Value::from(1));
    /// let shared = FieldStorage::from_shared(field);
    ///
    /// assert_eq!(shared.shared_count(), Some(1));
    ///
    /// let shared2 = shared.clone();
    /// assert_eq!(shared.shared_count(), Some(2));
    /// ```
    pub fn shared_count(&self) -> Option<usize> {
        match self {
            FieldStorage::Shared(arc) => Some(Arc::strong_count(arc)),
            FieldStorage::Owned(_) => None,
        }
    }
}

// Implement Display by delegating to inner field
impl Display for FieldStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_field().fmt(f)
    }
}

// Implement PartialEq by comparing the underlying fields
impl PartialEq for FieldStorage {
    fn eq(&self, other: &Self) -> bool {
        self.as_field().eq(other.as_field())
    }
}

impl Eq for FieldStorage {}

// Serialize: Transparently serialize as Field<Value>
impl Serialize for FieldStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Both variants serialize the same way (as Field<Value>)
        self.as_field().serialize(serializer)
    }
}

// Deserialize: Always deserialize to Owned variant
impl<'de> Deserialize<'de> for FieldStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize as Field<Value> and wrap in Owned variant
        Field::<Value>::deserialize(deserializer).map(FieldStorage::Owned)
    }
}

// Implement RecordItem trait by delegating to the inner field
impl RecordItem for FieldStorage {
    fn get_name(&self) -> &str {
        self.as_field().get_name()
    }

    fn get_meta(&self) -> &DataType {
        self.as_field().get_meta()
    }

    fn get_value(&self) -> &Value {
        self.as_field().get_value()
    }

    fn get_value_mut(&mut self) -> &mut Value {
        match self {
            FieldStorage::Shared(arc) => {
                // If there are multiple references, we need to clone to get a mutable version
                // This converts Shared to Owned variant
                let field = Arc::try_unwrap(std::mem::replace(
                    arc,
                    Arc::new(Field::new(DataType::Ignore, "", Value::from(false))),
                ))
                .unwrap_or_else(|arc| (*arc).clone());
                *self = FieldStorage::Owned(field);
                match self {
                    FieldStorage::Owned(f) => f.get_value_mut(),
                    _ => unreachable!(),
                }
            }
            FieldStorage::Owned(field) => field.get_value_mut(),
        }
    }
}

// Implement RecordItemFactory trait
impl RecordItemFactory for FieldStorage {
    fn from_digit<S: Into<FNameStr>>(name: S, val: i64) -> Self {
        FieldStorage::Owned(Field::from_digit(name, val))
    }

    fn from_ip<S: Into<FNameStr>>(name: S, ip: IpAddr) -> Self {
        FieldStorage::Owned(Field::from_ip(name, ip))
    }

    fn from_chars<N: Into<FNameStr>, Val: Into<FValueStr>>(name: N, val: Val) -> Self {
        FieldStorage::Owned(Field::from_chars(name, val))
    }
}

// Implement LevelFormatAble by delegating to inner field
impl LevelFormatAble for FieldStorage {
    fn level_fmt(&self, f: &mut Formatter<'_>, level: usize) -> std::fmt::Result {
        self.as_field().level_fmt(f, level)
    }
}

// 自动转换（降低迁移成本）
impl From<Field<Value>> for FieldStorage {
    fn from(field: Field<Value>) -> Self {
        FieldStorage::Owned(field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DataType;

    #[test]
    fn test_field_storage_shared_variant() {
        let field = Field::new(DataType::Chars, "test", Value::from("hello"));
        let storage = FieldStorage::Shared(Arc::new(field.clone()));

        // as_field returns correct reference
        assert_eq!(storage.as_field().get_name(), "test");
        assert!(storage.is_shared());
        assert_eq!(storage.shared_count(), Some(1));

        // Clone Shared variant only increments reference count
        let storage2 = storage.clone();
        assert_eq!(storage.shared_count(), Some(2));
        assert_eq!(storage2.shared_count(), Some(2));
    }

    #[test]
    fn test_field_storage_owned_variant() {
        let field = Field::new(DataType::Digit, "test", Value::from(42));
        let storage = FieldStorage::Owned(field);

        assert_eq!(storage.as_field().get_name(), "test");
        assert!(!storage.is_shared());
        assert_eq!(storage.shared_count(), None);
    }

    #[test]
    fn test_into_owned() {
        // Shared variant
        let field1 = Field::new(DataType::Chars, "shared_field", Value::from("value"));
        let storage1 = FieldStorage::Shared(Arc::new(field1));
        let owned1 = storage1.into_owned();
        assert_eq!(owned1.get_name(), "shared_field");

        // Owned variant
        let field2 = Field::new(DataType::Digit, "owned_field", Value::from(123));
        let storage2 = FieldStorage::Owned(field2);
        let owned2 = storage2.into_owned();
        assert_eq!(owned2.get_name(), "owned_field");
    }

    #[test]
    fn test_from_shared() {
        let field = Field::new(DataType::Chars, "name", Value::from("Alice"));
        let storage = FieldStorage::from_shared(field);

        assert!(storage.is_shared());
        assert_eq!(storage.as_field().get_name(), "name");
    }

    #[test]
    fn test_from_owned() {
        let field = Field::new(DataType::Digit, "count", Value::from(10));
        let storage = FieldStorage::from_owned(field);

        assert!(!storage.is_shared());
        assert_eq!(storage.as_field().get_name(), "count");
    }

    #[test]
    fn test_display() {
        let field = Field::new(DataType::Digit, "num", Value::from(42));
        let storage = FieldStorage::from_owned(field);

        let display = format!("{}", storage);
        assert!(display.contains("42"));
    }

    #[test]
    fn test_equality() {
        let field1 = Field::new(DataType::Chars, "test", Value::from("value"));
        let field2 = Field::new(DataType::Chars, "test", Value::from("value"));
        let field3 = Field::new(DataType::Chars, "test", Value::from("different"));

        let shared1 = FieldStorage::from_shared(field1.clone());
        let owned1 = FieldStorage::from_owned(field1);
        let shared2 = FieldStorage::from_shared(field2);
        let owned3 = FieldStorage::from_owned(field3);

        // Same content should be equal regardless of storage type
        assert_eq!(shared1, owned1);
        assert_eq!(shared1, shared2);

        // Different content should not be equal
        assert_ne!(shared1, owned3);
    }

    #[test]
    fn test_serde_serialization() {
        let field1 = Field::new(DataType::Chars, "f1", Value::from("shared"));
        let field2 = Field::new(DataType::Digit, "f2", Value::from(99));

        let shared = FieldStorage::from_shared(field1);
        let owned = FieldStorage::from_owned(field2);

        // Serialize
        let json_shared = serde_json::to_string(&shared).unwrap();
        let json_owned = serde_json::to_string(&owned).unwrap();

        // Deserialize (should always get Owned variant)
        let deserialized_shared: FieldStorage = serde_json::from_str(&json_shared).unwrap();
        let deserialized_owned: FieldStorage = serde_json::from_str(&json_owned).unwrap();

        // Verify values are correct
        assert_eq!(deserialized_shared.as_field().get_name(), "f1");
        assert_eq!(deserialized_owned.as_field().get_name(), "f2");

        // Verify both are Owned variant after deserialization
        assert!(!deserialized_shared.is_shared());
        assert!(!deserialized_owned.is_shared());
    }

    #[test]
    fn test_clone_performance_difference() {
        // This is more of a documentation test showing the usage pattern
        use crate::model::FValueStr;
        let large_str = FValueStr::from("x".repeat(1000));
        let field = Field::new(DataType::Chars, "large", Value::from(large_str));

        // Shared: cheap clone
        let shared = FieldStorage::from_shared(field.clone());
        let _shared2 = shared.clone();
        assert_eq!(shared.shared_count(), Some(2));

        // Owned: deep clone
        let owned = FieldStorage::from_owned(field);
        let _owned2 = owned.clone();
        assert!(owned.shared_count().is_none());
    }
}
