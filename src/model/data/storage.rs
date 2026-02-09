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

/// Storage mode for the underlying field value.
#[derive(Clone, Debug)]
pub enum ValueStorage {
    /// Arc shared (zero-copy passing)
    Shared(Arc<Field<Value>>),

    /// Exclusive ownership
    Owned(Field<Value>),
}

/// Field storage with zero-copy name override support.
///
/// Design principles:
/// - `cur_name` acts as an overlay, never modifying the underlying DataField
/// - Name lookup priority: cur_name > field.name
/// - `set_name()` only modifies cur_name, achieving zero-copy
///
/// # Performance
/// - Cloning `Shared` variant: ~5ns (Arc reference count increment)
/// - Cloning `Owned` variant: 50-500ns (deep copy of Field<Value>)
/// - Accessing field via `as_field()`: ~0-1ns
/// - `set_name()`: ~0ns (only sets cur_name, no field clone)
///
/// # Examples
///
/// ```ignore
/// use std::sync::Arc;
/// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
///
/// // Create a shared field (for static/constant values)
/// let static_field = Arc::new(Field::new(DataType::Chars, "app_name", Value::from("myapp")));
/// let shared = FieldStorage::from_shared(static_field);
///
/// // Zero-copy rename
/// let mut renamed = shared.clone();
/// renamed.set_name("application_name");  // Only sets cur_name, no clone!
///
/// // Create an owned field (for dynamic values)
/// let dynamic_field = Field::new(DataType::Digit, "counter", Value::from(42));
/// let owned = FieldStorage::from_owned(dynamic_field);
/// ```
#[derive(Clone, Debug)]
pub struct FieldStorage {
    /// Current field name (overrides field.name).
    /// None means use the underlying field's original name.
    cur_name: Option<FNameStr>,

    /// Field value storage
    value: ValueStorage,
}

impl FieldStorage {
    /// Create from Arc<DataField> as Shared variant (zero-copy).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::sync::Arc;
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Arc::new(Field::new(DataType::Chars, "name", Value::from("Alice")));
    /// let storage = FieldStorage::from_shared(field);
    ///
    /// assert!(storage.is_shared());
    /// ```
    #[inline]
    pub fn from_shared(field: Arc<Field<Value>>) -> Self {
        Self {
            cur_name: None,
            value: ValueStorage::Shared(field),
        }
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
    /// assert!(storage.is_owned());
    /// ```
    #[inline]
    pub fn from_owned(field: Field<Value>) -> Self {
        Self {
            cur_name: None,
            value: ValueStorage::Owned(field),
        }
    }

    /// Zero-copy set field name.
    ///
    /// Only modifies `cur_name`, does not affect the underlying Arc<DataField>.
    /// This is the core method of the zero-copy optimization!
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use std::sync::Arc;
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let field = Arc::new(Field::new(DataType::Chars, "HOST", Value::from("192.168.1.1")));
    /// let mut storage = FieldStorage::from_shared(Arc::clone(&field));
    /// storage.set_name("server_ip");  // Zero-copy!
    ///
    /// assert_eq!(storage.get_name(), "server_ip");
    /// assert_eq!(field.get_name(), "HOST");  // Original unchanged
    /// ```
    #[inline]
    pub fn set_name(&mut self, name: impl Into<FNameStr>) {
        self.cur_name = Some(name.into());
    }

    /// Get the current effective field name.
    ///
    /// Priority: cur_name > field.name
    #[inline]
    pub fn get_name(&self) -> &str {
        if let Some(ref name) = self.cur_name {
            name.as_str()
        } else {
            self.as_field().get_name()
        }
    }

    /// Get a reference to the underlying field (unified interface).
    ///
    /// Note: This returns the underlying field, whose name may differ from
    /// `get_name()` if `set_name()` was called.
    ///
    /// # Performance
    /// - Shared variant: ~1ns (dereference)
    /// - Owned variant: 0ns (direct reference)
    #[inline]
    pub fn as_field(&self) -> &Field<Value> {
        match &self.value {
            ValueStorage::Shared(arc) => arc.as_ref(),
            ValueStorage::Owned(field) => field,
        }
    }

    /// Convert to owned Field<Value>.
    ///
    /// If `cur_name` is set, it will be applied to the returned field.
    /// If `cur_name` is None and this is an Owned variant, returns without cloning.
    ///
    /// # Performance
    /// - Shared variant: ~50-500ns (clone the inner field if multiple references exist)
    /// - Owned variant: 0ns (move)
    pub fn into_owned(self) -> Field<Value> {
        let mut field = match self.value {
            ValueStorage::Shared(arc) => {
                // Try to unwrap if this is the only reference, otherwise clone
                Arc::try_unwrap(arc).unwrap_or_else(|arc| (*arc).clone())
            }
            ValueStorage::Owned(field) => field,
        };

        // Apply cur_name override
        if let Some(name) = self.cur_name {
            field.set_name(name);
        }

        field
    }

    /// Check if this is a Shared variant.
    #[inline]
    pub fn is_shared(&self) -> bool {
        matches!(self.value, ValueStorage::Shared(_))
    }

    /// Check if this is an Owned variant.
    #[inline]
    pub fn is_owned(&self) -> bool {
        matches!(self.value, ValueStorage::Owned(_))
    }

    /// Get Arc reference count (for debugging/diagnostics).
    ///
    /// Returns `Some(count)` for Shared variant, `None` for Owned variant.
    pub fn shared_count(&self) -> Option<usize> {
        match &self.value {
            ValueStorage::Shared(arc) => Some(Arc::strong_count(arc)),
            ValueStorage::Owned(_) => None,
        }
    }

    /// Get a mutable reference to the underlying field.
    ///
    /// For the `Shared` variant, this converts to `Owned` first (clone-on-write).
    /// If `cur_name` is set, it will be applied to the field before returning.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{FieldStorage, Field, Value, DataType};
    ///
    /// let mut storage = FieldStorage::from_owned(Field::from_chars("name", "Alice"));
    /// storage.as_field_mut().set_name("renamed");
    /// assert_eq!(storage.as_field().get_name(), "renamed");
    /// ```
    pub fn as_field_mut(&mut self) -> &mut Field<Value> {
        // Convert Shared to Owned if needed
        if let ValueStorage::Shared(_) = self.value {
            let old_value = std::mem::replace(
                &mut self.value,
                ValueStorage::Owned(Field::new(DataType::Ignore, "", Value::from(false))),
            );
            let field = match old_value {
                ValueStorage::Shared(arc) => {
                    Arc::try_unwrap(arc).unwrap_or_else(|arc| (*arc).clone())
                }
                ValueStorage::Owned(field) => field,
            };
            self.value = ValueStorage::Owned(field);
        }

        // Apply cur_name if set
        if let Some(name) = self.cur_name.take()
            && let ValueStorage::Owned(ref mut field) = self.value
        {
            field.set_name(name);
        }

        match &mut self.value {
            ValueStorage::Owned(field) => field,
            ValueStorage::Shared(_) => unreachable!(),
        }
    }

    /// Get field value reference.
    #[inline]
    pub fn get_value(&self) -> &Value {
        self.as_field().get_value()
    }

    /// Get type metadata.
    #[inline]
    pub fn get_meta(&self) -> &DataType {
        self.as_field().get_meta()
    }
}

// Implement Display by delegating to inner field
impl Display for FieldStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_field().fmt(f)
    }
}

// Implement PartialEq by comparing effective names and field contents
impl PartialEq for FieldStorage {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
            && self.as_field().get_meta() == other.as_field().get_meta()
            && self.as_field().get_value() == other.as_field().get_value()
    }
}

impl Eq for FieldStorage {}

// Serialize: Transparently serialize as Field<Value> with effective name
impl Serialize for FieldStorage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ref name) = self.cur_name {
            // Serialize with the effective name
            let mut field = self.as_field().clone();
            field.set_name(name.as_str());
            field.serialize(serializer)
        } else {
            self.as_field().serialize(serializer)
        }
    }
}

// Deserialize: Always deserialize to Owned variant
impl<'de> Deserialize<'de> for FieldStorage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Field::<Value>::deserialize(deserializer).map(FieldStorage::from_owned)
    }
}

// Implement RecordItem trait by delegating to the inner field
impl RecordItem for FieldStorage {
    fn get_name(&self) -> &str {
        if let Some(ref name) = self.cur_name {
            name.as_str()
        } else {
            self.as_field().get_name()
        }
    }

    fn get_meta(&self) -> &DataType {
        self.as_field().get_meta()
    }

    fn get_value(&self) -> &Value {
        self.as_field().get_value()
    }

    fn get_value_mut(&mut self) -> &mut Value {
        self.as_field_mut().get_value_mut()
    }
}

// Implement RecordItemFactory trait
impl RecordItemFactory for FieldStorage {
    fn from_digit<S: Into<FNameStr>>(name: S, val: i64) -> Self {
        FieldStorage::from_owned(Field::from_digit(name, val))
    }

    fn from_ip<S: Into<FNameStr>>(name: S, ip: IpAddr) -> Self {
        FieldStorage::from_owned(Field::from_ip(name, ip))
    }

    fn from_chars<N: Into<FNameStr>, Val: Into<FValueStr>>(name: N, val: Val) -> Self {
        FieldStorage::from_owned(Field::from_chars(name, val))
    }
}

// Implement LevelFormatAble
impl LevelFormatAble for FieldStorage {
    fn level_fmt(&self, f: &mut Formatter<'_>, level: usize) -> std::fmt::Result {
        if let Some(ref name) = self.cur_name {
            let meta: String = From::from(self.as_field().get_meta());
            writeln!(
                f,
                "{:width$}[{:<16}] {:<20} : {}",
                "",
                meta,
                name,
                self.as_field().value,
                width = level * 6
            )
        } else {
            self.as_field().level_fmt(f, level)
        }
    }
}

// Auto conversion (reduce migration cost)
impl From<Field<Value>> for FieldStorage {
    fn from(field: Field<Value>) -> Self {
        FieldStorage::from_owned(field)
    }
}

impl From<Arc<Field<Value>>> for FieldStorage {
    fn from(arc: Arc<Field<Value>>) -> Self {
        FieldStorage::from_shared(arc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::DataType;

    #[test]
    fn test_field_storage_shared_variant() {
        let field = Field::new(DataType::Chars, "test", Value::from("hello"));
        let storage = FieldStorage::from_shared(Arc::new(field.clone()));

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
        let storage = FieldStorage::from_owned(field);

        assert_eq!(storage.as_field().get_name(), "test");
        assert!(!storage.is_shared());
        assert_eq!(storage.shared_count(), None);
    }

    #[test]
    fn test_into_owned() {
        // Shared variant
        let field1 = Field::new(DataType::Chars, "shared_field", Value::from("value"));
        let storage1 = FieldStorage::from_shared(Arc::new(field1));
        let owned1 = storage1.into_owned();
        assert_eq!(owned1.get_name(), "shared_field");

        // Owned variant
        let field2 = Field::new(DataType::Digit, "owned_field", Value::from(123));
        let storage2 = FieldStorage::from_owned(field2);
        let owned2 = storage2.into_owned();
        assert_eq!(owned2.get_name(), "owned_field");
    }

    #[test]
    fn test_from_shared() {
        let field = Field::new(DataType::Chars, "name", Value::from("Alice"));
        let storage = FieldStorage::from_shared(Arc::new(field));

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

        let shared1 = FieldStorage::from_shared(Arc::new(field1.clone()));
        let owned1 = FieldStorage::from_owned(field1);
        let shared2 = FieldStorage::from_shared(Arc::new(field2));
        let owned3 = FieldStorage::from_owned(field3);

        // Same content should be equal regardless of storage type
        assert_eq!(shared1, owned1);
        assert_eq!(shared1, shared2);

        // Different content should not be equal
        assert_ne!(shared1, owned3);
    }

    #[test]
    fn test_equality_with_cur_name() {
        let field = Field::new(DataType::Chars, "test", Value::from("value"));

        let mut storage1 = FieldStorage::from_owned(field.clone());
        storage1.set_name("renamed");

        let storage2 = FieldStorage::from_owned(field);

        // Different effective names should not be equal
        assert_ne!(storage1, storage2);
    }

    #[test]
    fn test_serde_serialization() {
        let field1 = Field::new(DataType::Chars, "f1", Value::from("shared"));
        let field2 = Field::new(DataType::Digit, "f2", Value::from(99));

        let shared = FieldStorage::from_shared(Arc::new(field1));
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
    fn test_serde_with_cur_name() {
        let field = Field::new(DataType::Chars, "original", Value::from("value"));
        let mut storage = FieldStorage::from_owned(field);
        storage.set_name("renamed");

        // Serialize should use the effective name
        let json = serde_json::to_string(&storage).unwrap();
        let deserialized: FieldStorage = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.get_name(), "renamed");
    }

    #[test]
    fn test_clone_performance_difference() {
        // This is more of a documentation test showing the usage pattern
        use crate::model::FValueStr;
        let large_str = FValueStr::from("x".repeat(1000));
        let field = Field::new(DataType::Chars, "large", Value::from(large_str));

        // Shared: cheap clone
        let shared = FieldStorage::from_shared(Arc::new(field.clone()));
        let _shared2 = shared.clone();
        assert_eq!(shared.shared_count(), Some(2));

        // Owned: deep clone
        let owned = FieldStorage::from_owned(field);
        let _owned2 = owned.clone();
        assert!(owned.shared_count().is_none());
    }

    #[test]
    fn test_from_arc_to_fieldstorage() {
        let field = Field::new(DataType::Chars, "name", Value::from("Alice"));
        let arc = Arc::new(field);
        let storage: FieldStorage = arc.into();

        assert!(storage.is_shared());
        assert_eq!(storage.as_field().get_name(), "name");
    }

    #[test]
    fn test_is_owned() {
        let field = Field::new(DataType::Digit, "x", Value::from(1));
        let owned = FieldStorage::from_owned(field.clone());
        let shared = FieldStorage::from_shared(Arc::new(field));

        assert!(owned.is_owned());
        assert!(!shared.is_owned());
    }

    #[test]
    fn test_as_field_mut_owned() {
        let mut storage = FieldStorage::from_owned(Field::from_chars("name", "Alice"));
        storage.as_field_mut().set_name("renamed");
        assert_eq!(storage.as_field().get_name(), "renamed");
        assert!(storage.is_owned());
    }

    #[test]
    fn test_as_field_mut_shared() {
        let field = Field::from_chars("name", "Alice");
        let mut storage = FieldStorage::from_shared(Arc::new(field));
        assert!(storage.is_shared());

        // Mutating converts Shared to Owned
        storage.as_field_mut().set_name("renamed");
        assert!(storage.is_owned());
        assert_eq!(storage.as_field().get_name(), "renamed");
    }

    #[test]
    fn test_as_field_mut_applies_cur_name() {
        let field = Field::from_chars("original", "value");
        let mut storage = FieldStorage::from_owned(field);
        storage.set_name("override");

        // as_field_mut should apply cur_name
        let field_mut = storage.as_field_mut();
        assert_eq!(field_mut.get_name(), "override");

        // After as_field_mut, cur_name should be consumed
        assert_eq!(storage.as_field().get_name(), "override");
    }

    // ===== Zero-copy specific tests =====

    #[test]
    fn test_zero_copy_set_name() {
        // Create Arc field
        let field = Arc::new(Field::from_chars("original", "value"));
        let original_count = Arc::strong_count(&field);

        // Create Shared storage
        let mut storage = FieldStorage::from_shared(Arc::clone(&field));
        assert_eq!(Arc::strong_count(&field), original_count + 1);

        // Zero-copy set name
        storage.set_name("renamed");

        // Verify: Arc reference count unchanged (no clone happened)
        assert_eq!(Arc::strong_count(&field), original_count + 1);

        // Verify: name updated
        assert_eq!(storage.get_name(), "renamed");

        // Verify: underlying field name unchanged
        assert_eq!(field.get_name(), "original");

        // Verify: value unchanged
        assert_eq!(storage.get_value().as_str().unwrap(), "value");
    }

    #[test]
    fn test_name_priority() {
        let field = Field::from_chars("field_name", "value");
        let mut storage = FieldStorage::from_owned(field);

        // Initial: use field.name
        assert_eq!(storage.get_name(), "field_name");

        // Set cur_name
        storage.set_name("override_name");

        // cur_name takes priority
        assert_eq!(storage.get_name(), "override_name");

        // Underlying field name unchanged
        assert_eq!(storage.as_field().get_name(), "field_name");
    }

    #[test]
    fn test_into_owned_applies_cur_name() {
        let field = Arc::new(Field::from_chars("original", "value"));
        let mut storage = FieldStorage::from_shared(field);

        storage.set_name("renamed");

        let owned = storage.into_owned();

        // Verify: name applied
        assert_eq!(owned.get_name(), "renamed");
        assert_eq!(owned.get_value().as_str().unwrap(), "value");
    }

    #[test]
    fn test_shared_vs_owned() {
        let arc_field = Arc::new(Field::from_chars("arc", "value1"));
        let owned_field = Field::from_chars("owned", "value2");

        let shared = FieldStorage::from_shared(arc_field);
        let owned = FieldStorage::from_owned(owned_field);

        assert!(shared.is_shared());
        assert!(!shared.is_owned());

        assert!(!owned.is_shared());
        assert!(owned.is_owned());
    }

    #[test]
    fn test_arc_strong_count() {
        let field = Arc::new(Field::from_chars("test", "value"));
        let initial_count = Arc::strong_count(&field);

        let storage1 = FieldStorage::from_shared(Arc::clone(&field));
        assert_eq!(storage1.shared_count(), Some(initial_count + 1));

        let storage2 = FieldStorage::from_shared(Arc::clone(&field));
        assert_eq!(storage2.shared_count(), Some(initial_count + 2));

        let owned = FieldStorage::from_owned(Field::from_chars("test", "value"));
        assert_eq!(owned.shared_count(), None);
    }

    #[test]
    fn test_multi_stage_zero_copy() {
        // Simulate multi-stage processing
        let field = Arc::new(Field::from_chars("HOST", "192.168.1.1"));
        let initial_count = Arc::strong_count(&field);

        // Stage 1: rename to server_ip
        let mut stage1 = FieldStorage::from_shared(Arc::clone(&field));
        stage1.set_name("server_ip");

        // Stage 2: rename to host_address
        let mut stage2 = FieldStorage::from_shared(Arc::clone(&field));
        stage2.set_name("host_address");

        // Stage 3: rename to ip_addr
        let mut stage3 = FieldStorage::from_shared(Arc::clone(&field));
        stage3.set_name("ip_addr");

        // Verify: all stages share the same Arc
        assert_eq!(stage1.shared_count(), Some(initial_count + 3));
        assert_eq!(stage2.shared_count(), Some(initial_count + 3));
        assert_eq!(stage3.shared_count(), Some(initial_count + 3));

        // Verify: each stage has independent name
        assert_eq!(stage1.get_name(), "server_ip");
        assert_eq!(stage2.get_name(), "host_address");
        assert_eq!(stage3.get_name(), "ip_addr");

        // Verify: all stages share the same value
        assert_eq!(stage1.get_value().as_str().unwrap(), "192.168.1.1");
        assert_eq!(stage2.get_value().as_str().unwrap(), "192.168.1.1");
        assert_eq!(stage3.get_value().as_str().unwrap(), "192.168.1.1");
    }

    #[test]
    fn test_record_item_get_name_with_cur_name() {
        let field = Field::from_chars("original", "value");
        let mut storage = FieldStorage::from_owned(field);
        storage.set_name("overridden");

        // RecordItem::get_name should return the effective name
        let name: &str = RecordItem::get_name(&storage);
        assert_eq!(name, "overridden");
    }
}
