use crate::model::format::LevelFormatAble;
use crate::model::{DataType, FNameStr, FValueStr, Value};
use crate::model::{FieldRef, Maker};
use crate::traits::AsValueRef;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use super::field::Field;
use super::storage::FieldStorage;
pub const WP_EVENT_ID: &str = "wp_event_id";
/// 记录中每一项需要暴露的行为
pub trait RecordItem {
    fn get_name(&self) -> &str;
    fn get_meta(&self) -> &DataType;
    fn get_value(&self) -> &Value;
    fn get_value_mut(&mut self) -> &mut Value;
}

/// 为 Record 生成字段所需的工厂方法
pub trait RecordItemFactory {
    fn from_digit<S: Into<FNameStr>>(name: S, val: i64) -> Self;
    fn from_ip<S: Into<FNameStr>>(name: S, ip: IpAddr) -> Self;
    fn from_chars<N: Into<FNameStr>, Val: Into<FValueStr>>(name: N, val: Val) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record<T> {
    pub id: u64,
    pub items: Vec<T>,
}

impl<T> Default for Record<T> {
    fn default() -> Self {
        Self {
            id: 0,
            items: Vec::with_capacity(10),
        }
    }
}

impl<T> From<Vec<T>> for Record<T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            id: 0,
            items: value,
        }
    }
}

impl<T> Display for Record<T>
where
    T: RecordItem + LevelFormatAble,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (i, o) in self.items.iter().enumerate() {
            if *o.get_meta() != DataType::Ignore {
                write!(f, "NO:{:<5}", i + 1)?;
                o.level_fmt(f, 1)?;
            }
        }
        Ok(())
    }
}

impl<T> Record<T>
where
    T: RecordItem + RecordItemFactory,
{
    pub fn set_id(&mut self, id: u64) {
        // 设置 id 字段
        self.id = id;

        // 如果已存在 wp_event_id 字段，避免重复追加
        if self.items.iter().any(|f| f.get_name() == WP_EVENT_ID) {
            return;
        }
        let Ok(id_i64) = i64::try_from(id) else {
            // 事件 ID 超出 i64 无法表示，保持记录原状
            return;
        };
        self.items.insert(0, T::from_digit(WP_EVENT_ID, id_i64));
    }
    pub fn test_value() -> Self {
        let data = vec![
            T::from_ip("ip", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            T::from_chars("chars", "test"),
        ];
        Self { id: 0, items: data }
    }
}

impl<T> Record<T>
where
    T: RecordItem,
{
    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.items
            .iter()
            .find(|x| x.get_name() == key)
            .map(|x| x.get_value())
    }
}

impl<T> Record<T> {
    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Append a field to the record.
    ///
    /// Accepts any type that can be converted into `T`. For `DataRecord`,
    /// this means you can pass [`DataField`], [`FieldStorage`], or `Arc<DataField>` directly.
    pub fn append(&mut self, data: impl Into<T>) {
        self.items.push(data.into());
    }

    /// Alias for [`append`](Self::append) following Vec naming convention.
    pub fn push(&mut self, data: impl Into<T>) {
        self.items.push(data.into());
    }

    pub fn merge(&mut self, mut other: Self) {
        self.items.append(&mut other.items);
    }
}

impl<T> Record<T>
where
    T: RecordItem,
{
    // 存在同名字段时取第一个字段返回值
    pub fn field(&self, key: &str) -> Option<&T> {
        self.items.iter().find(|item| item.get_name() == key)
    }

    pub fn field_mut(&mut self, key: &str) -> Option<&mut T> {
        self.items.iter_mut().find(|item| item.get_name() == key)
    }

    pub fn get2(&self, name: &str) -> Option<&T> {
        self.items.iter().find(|x| x.get_name() == name)
    }
    pub fn get_value_mut(&mut self, name: &str) -> Option<&mut T> {
        self.items.iter_mut().find(|x| x.get_name() == name)
    }
    pub fn remove_field(&mut self, name: &str) -> bool {
        let pos = self.items.iter().position(|x| x.get_name() == name);
        if let Some(pos) = pos {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }
}

impl<V> RecordItem for Field<V>
where
    V: AsValueRef<Value>,
{
    fn get_name(&self) -> &str {
        Field::get_name(self)
    }

    fn get_meta(&self) -> &DataType {
        Field::get_meta(self)
    }

    fn get_value(&self) -> &Value {
        Field::get_value(self)
    }

    fn get_value_mut(&mut self) -> &mut Value {
        Field::get_value_mut(self)
    }
}

impl<V> RecordItemFactory for Field<V>
where
    V: Maker<i64> + Maker<FValueStr> + Maker<IpAddr>,
{
    fn from_digit<S: Into<FNameStr>>(name: S, val: i64) -> Self {
        Field::from_digit(name, val)
    }

    fn from_ip<S: Into<FNameStr>>(name: S, ip: IpAddr) -> Self {
        Field::from_ip(name, ip)
    }

    fn from_chars<N: Into<FNameStr>, Val: Into<FValueStr>>(name: N, val: Val) -> Self {
        Field::from_chars(name, val)
    }
}

// Convenience construction for Record<FieldStorage> from Vec<DataField>
impl From<Vec<Field<Value>>> for Record<FieldStorage> {
    fn from(fields: Vec<Field<Value>>) -> Self {
        let items: Vec<FieldStorage> = fields.into_iter().map(FieldStorage::from_owned).collect();
        Self { id: 0, items }
    }
}

// Convenience construction for Record<FieldStorage> from a single DataField
impl From<Field<Value>> for Record<FieldStorage> {
    fn from(field: Field<Value>) -> Self {
        Self {
            id: 0,
            items: vec![FieldStorage::from_owned(field)],
        }
    }
}

// Convenience methods for Record<FieldStorage> (DataRecord with mixed storage)
impl Record<FieldStorage> {
    /// Push a shared field (Arc-wrapped) to the record
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use wp_model_core::model::{DataRecord, Field, Value, DataType, FieldStorage};
    ///
    /// let mut record = DataRecord::default();
    /// let field = Arc::new(Field::new(DataType::Chars, "app_name", Value::from("myapp")));
    /// record.push_shared(field);
    /// ```
    pub fn push_shared(&mut self, field: Arc<Field<Value>>) {
        self.items.push(FieldStorage::from_shared(field));
    }

    /// Push an owned field to the record
    ///
    /// # Examples
    ///
    /// ```
    /// use wp_model_core::model::{DataRecord, Field, Value, DataType};
    ///
    /// let mut record = DataRecord::default();
    /// let field = Field::new(DataType::Digit, "count", Value::from(42));
    /// record.push_owned(field);
    /// ```
    pub fn push_owned(&mut self, field: Field<Value>) {
        self.items.push(FieldStorage::from_owned(field));
    }

    /// Get a reference to the underlying field by index
    ///
    /// # Examples
    ///
    /// ```
    /// use wp_model_core::model::{DataRecord, Field, Value, DataType};
    ///
    /// let mut record = DataRecord::default();
    /// let field = Field::new(DataType::Digit, "x", Value::from(10));
    /// record.push_owned(field);
    ///
    /// let retrieved = record.field_at(0);
    /// assert!(retrieved.is_some());
    /// assert_eq!(retrieved.unwrap().get_name(), "x");
    /// ```
    pub fn field_at(&self, index: usize) -> Option<&Field<Value>> {
        self.items.get(index).map(|s| s.as_field())
    }

    /// Get a [`FieldRef`] by name (zero-copy, cur_name-aware).
    ///
    /// Returns a [`FieldRef`] that respects the `cur_name` override.
    /// For owned field access, use [`get_field_owned()`](Self::get_field_owned).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{DataRecord, DataField};
    ///
    /// let record = DataRecord::from(vec![
    ///     DataField::from_chars("name", "Alice"),
    /// ]);
    /// assert_eq!(record.get_field("name").map(|f| f.get_name()), Some("name"));
    /// assert!(record.get_field("missing").is_none());
    /// ```
    pub fn get_field(&self, name: &str) -> Option<FieldRef<'_>> {
        self.field(name).map(|s| s.field_ref())
    }

    /// Get owned field by name with cur_name applied.
    ///
    /// Use when you need to modify the field or transfer ownership.
    pub fn get_field_owned(&self, name: &str) -> Option<Field<Value>> {
        self.get_field(name).map(|f| f.to_owned())
    }

    /// Get a mutable reference to the underlying [`DataField`] by name.
    ///
    /// For `Shared` fields, this converts to `Owned` first (clone-on-write).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use wp_model_core::model::{DataRecord, DataField};
    ///
    /// let mut record = DataRecord::from(vec![
    ///     DataField::from_chars("name", "Alice"),
    /// ]);
    /// if let Some(field) = record.get_field_mut("name") {
    ///     field.set_name("renamed");
    /// }
    /// ```
    pub fn get_field_mut(&mut self, name: &str) -> Option<&mut Field<Value>> {
        self.field_mut(name).map(|s| s.as_field_mut())
    }

    /// Iterate over all fields as [`DataField`] references.
    ///
    /// Note: Returns underlying field references. Field names may not reflect
    /// `cur_name` overrides. Use [`field_refs()`](Self::field_refs) for cur_name-aware iteration.
    pub fn fields(&self) -> impl Iterator<Item = &Field<Value>> + '_ {
        self.items.iter().map(|s| s.as_field())
    }

    /// Iterate over all fields as [`FieldRef`] (zero-copy, cur_name-aware).
    pub fn field_refs(&self) -> impl Iterator<Item = FieldRef<'_>> + '_ {
        self.items.iter().map(|s| s.field_ref())
    }

    /// Iterate over all fields as mutable [`DataField`] references.
    ///
    /// For `Shared` fields, this converts them to `Owned` (clone-on-write).
    pub fn fields_mut(&mut self) -> impl Iterator<Item = &mut Field<Value>> + '_ {
        self.items.iter_mut().map(|s| s.as_field_mut())
    }

    /// Get statistics about storage types (shared vs owned)
    ///
    /// Returns (shared_count, owned_count)
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use wp_model_core::model::{DataRecord, Field, Value, DataType};
    ///
    /// let mut record = DataRecord::default();
    /// let static_field = Arc::new(Field::new(DataType::Chars, "static", Value::from("val")));
    /// record.push_shared(static_field);
    ///
    /// let dynamic_field = Field::new(DataType::Digit, "dynamic", Value::from(10));
    /// record.push_owned(dynamic_field);
    ///
    /// let (shared, owned) = record.storage_stats();
    /// assert_eq!(shared, 1);
    /// assert_eq!(owned, 1);
    /// ```
    pub fn storage_stats(&self) -> (usize, usize) {
        let mut shared_count = 0;
        let mut owned_count = 0;
        for item in &self.items {
            if item.is_shared() {
                shared_count += 1;
            } else {
                owned_count += 1;
            }
        }
        (shared_count, owned_count)
    }

    /// Convert to a fully owned record (Record<Field<Value>>)
    ///
    /// This is useful when you need to pass the record to code that expects
    /// the old Record<Field<Value>> type.
    ///
    /// # Examples
    ///
    /// ```
    /// use wp_model_core::model::{DataRecord, Field, Value, DataType};
    ///
    /// let mut record = DataRecord::default();
    /// let field = Field::new(DataType::Digit, "x", Value::from(10));
    /// record.push_owned(field);
    ///
    /// let owned_record = record.into_owned_record();
    /// assert_eq!(owned_record.items.len(), 1);
    /// ```
    pub fn into_owned_record(self) -> Record<Field<Value>> {
        Record {
            id: self.id,
            items: self
                .items
                .into_iter()
                .map(|storage| storage.into_owned())
                .collect(),
        }
    }
}

// ValueGetter impl removed from core; use function-style adapters in extension crates.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DataField, DataRecord, FieldStorage};
    use std::net::Ipv4Addr;

    fn make_test_record() -> DataRecord {
        let fields = vec![
            FieldStorage::from_chars("name", "Alice"),
            FieldStorage::from_digit("age", 30),
            FieldStorage::from_ip("ip", IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
        ];
        Record::from(fields)
    }

    // ========== Record creation tests ==========

    #[test]
    fn test_record_default() {
        let record: DataRecord = Record::default();
        assert!(record.items.is_empty());
    }

    #[test]
    fn test_record_from_vec() {
        let fields: Vec<FieldStorage> = vec![
            FieldStorage::from_digit("x", 1),
            FieldStorage::from_digit("y", 2),
        ];
        let record: DataRecord = Record::from(fields);
        assert_eq!(record.items.len(), 2);
    }

    #[test]
    fn test_record_test_value() {
        let record: DataRecord = Record::test_value();
        assert_eq!(record.items.len(), 2);
        assert!(record.field("ip").is_some());
        assert!(record.field("chars").is_some());
    }

    // ========== Record field access tests ==========

    #[test]
    fn test_record_field() {
        let record = make_test_record();

        let name_field = record.field("name");
        assert!(name_field.is_some());
        assert_eq!(name_field.unwrap().get_name(), "name");

        let missing = record.field("missing");
        assert!(missing.is_none());
    }

    #[test]
    fn test_record_get2() {
        let record = make_test_record();

        let age_field = record.get2("age");
        assert!(age_field.is_some());
        assert_eq!(age_field.unwrap().get_meta(), &DataType::Digit);
    }

    #[test]
    fn test_record_get_value() {
        let record = make_test_record();

        let age_value = record.get_value("age");
        assert!(age_value.is_some());
        assert_eq!(age_value.unwrap(), &Value::Digit(30));

        let missing = record.get_value("missing");
        assert!(missing.is_none());
    }

    #[test]
    fn test_record_get_value_mut() {
        let mut record = make_test_record();

        let field = record.get_value_mut("age");
        assert!(field.is_some());

        // Modify the value through mutable reference
        if let Some(f) = field {
            *f.get_value_mut() = Value::Digit(31);
        }

        assert_eq!(record.get_value("age"), Some(&Value::Digit(31)));
    }

    // ========== Record mutation tests ==========

    #[test]
    fn test_record_append() {
        let mut record: DataRecord = Record::default();
        assert_eq!(record.items.len(), 0);

        record.append(FieldStorage::from_digit("count", 100));
        assert_eq!(record.items.len(), 1);

        record.append(FieldStorage::from_chars("msg", "hello"));
        assert_eq!(record.items.len(), 2);
    }

    #[test]
    fn test_record_merge() {
        let mut record1: DataRecord = Record::from(vec![FieldStorage::from_digit("a", 1)]);
        let record2: DataRecord = Record::from(vec![
            FieldStorage::from_digit("b", 2),
            FieldStorage::from_digit("c", 3),
        ]);

        record1.merge(record2);
        assert_eq!(record1.items.len(), 3);
        assert!(record1.field("a").is_some());
        assert!(record1.field("b").is_some());
        assert!(record1.field("c").is_some());
    }

    #[test]
    fn test_record_remove_field() {
        let mut record = make_test_record();
        assert_eq!(record.items.len(), 3);

        let removed = record.remove_field("age");
        assert!(removed);
        assert_eq!(record.items.len(), 2);
        assert!(record.field("age").is_none());

        let not_found = record.remove_field("nonexistent");
        assert!(!not_found);
        assert_eq!(record.items.len(), 2);
    }

    // ========== set_id tests ==========

    #[test]
    fn test_record_set_id() {
        let mut record = make_test_record();
        let original_len = record.items.len();

        record.set_id(12345);

        // ID should be set in both the id field and added to items
        assert_eq!(record.id, 12345);
        assert_eq!(record.items.len(), original_len + 1);
        // ID should be inserted at position 0
        assert_eq!(record.items[0].get_name(), WP_EVENT_ID);
        assert_eq!(record.items[0].get_value(), &Value::Digit(12345));
    }

    #[test]
    fn test_record_set_id_no_duplicate() {
        let mut record = make_test_record();

        record.set_id(100);
        assert_eq!(record.id, 100);
        let len_after_first = record.items.len();

        // Try to set ID again - should update id field but not add duplicate to items
        record.set_id(200);
        assert_eq!(record.id, 200);
        assert_eq!(record.items.len(), len_after_first);
        // Original ID in items should remain
        assert_eq!(record.get_value(WP_EVENT_ID), Some(&Value::Digit(100)));
    }

    // ========== RecordItem trait tests ==========

    #[test]
    fn test_field_as_record_item() {
        let field: DataField = Field::from_chars("key", "value");

        // Test RecordItem trait methods
        assert_eq!(field.get_name(), "key");
        assert_eq!(field.get_meta(), &DataType::Chars);
        assert_eq!(field.get_value(), &Value::Chars("value".into()));
    }

    #[test]
    fn test_field_record_item_get_value_mut() {
        let mut field: DataField = Field::from_digit("num", 10);

        *field.get_value_mut() = Value::Digit(20);
        assert_eq!(field.get_value(), &Value::Digit(20));
    }

    // ========== FieldStorage RecordItem tests ==========

    #[test]
    fn test_field_storage_as_record_item() {
        let storage: FieldStorage = FieldStorage::from_chars("key", "value");

        // Test RecordItem trait methods
        assert_eq!(storage.get_name(), "key");
        assert_eq!(storage.get_meta(), &DataType::Chars);
        assert_eq!(storage.get_value(), &Value::Chars("value".into()));
    }

    #[test]
    fn test_field_storage_record_item_get_value_mut() {
        let mut storage: FieldStorage = FieldStorage::from_digit("num", 10);

        *storage.get_value_mut() = Value::Digit(20);
        assert_eq!(storage.get_value(), &Value::Digit(20));
    }

    // ========== RecordItemFactory trait tests ==========

    #[test]
    fn test_record_item_factory() {
        let digit: FieldStorage = <FieldStorage as RecordItemFactory>::from_digit("n", 42);
        assert_eq!(digit.get_meta(), &DataType::Digit);

        let ip: FieldStorage = <FieldStorage as RecordItemFactory>::from_ip(
            "addr",
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
        );
        assert_eq!(ip.get_meta(), &DataType::IP);

        let chars: FieldStorage = <FieldStorage as RecordItemFactory>::from_chars("s", "hello");
        assert_eq!(chars.get_meta(), &DataType::Chars);
    }

    // ========== Display test ==========

    #[test]
    fn test_record_display() {
        let record = make_test_record();
        let display = format!("{}", record);

        assert!(display.contains("name"));
        assert!(display.contains("age"));
        assert!(display.contains("ip"));
    }

    // ========== Record<FieldStorage> convenience methods tests ==========

    #[test]
    fn test_push_shared() {
        let mut record = DataRecord::default();
        let field = Arc::new(Field::new(DataType::Chars, "static", Value::from("value")));

        record.push_shared(field.clone());
        assert_eq!(record.items.len(), 1);
        assert!(record.items[0].is_shared());
        assert_eq!(record.items[0].as_field().get_name(), "static");
    }

    #[test]
    fn test_push_owned() {
        let mut record = DataRecord::default();
        let field = Field::new(DataType::Digit, "dynamic", Value::from(42));

        record.push_owned(field);
        assert_eq!(record.items.len(), 1);
        assert!(!record.items[0].is_shared());
        assert_eq!(record.items[0].as_field().get_name(), "dynamic");
    }

    #[test]
    fn test_get_field() {
        let mut record = DataRecord::default();
        record.push_owned(Field::new(DataType::Chars, "test", Value::from("value")));

        let field = record.field_at(0);
        assert!(field.is_some());
        assert_eq!(field.unwrap().get_name(), "test");

        let missing = record.field_at(10);
        assert!(missing.is_none());
    }

    #[test]
    fn test_storage_stats() {
        let mut record = DataRecord::default();

        // Add shared fields
        record.push_shared(Arc::new(Field::new(
            DataType::Chars,
            "s1",
            Value::from("a"),
        )));
        record.push_shared(Arc::new(Field::new(
            DataType::Chars,
            "s2",
            Value::from("b"),
        )));

        // Add owned fields
        record.push_owned(Field::new(DataType::Digit, "o1", Value::from(1)));
        record.push_owned(Field::new(DataType::Digit, "o2", Value::from(2)));
        record.push_owned(Field::new(DataType::Digit, "o3", Value::from(3)));

        let (shared, owned) = record.storage_stats();
        assert_eq!(shared, 2);
        assert_eq!(owned, 3);
    }

    #[test]
    fn test_into_owned_record() {
        let mut record = DataRecord::default();
        record.push_shared(Arc::new(Field::new(
            DataType::Chars,
            "s",
            Value::from("shared"),
        )));
        record.push_owned(Field::new(DataType::Digit, "o", Value::from(10)));

        let owned_record = record.into_owned_record();
        assert_eq!(owned_record.items.len(), 2);
        assert_eq!(owned_record.items[0].get_name(), "s");
        assert_eq!(owned_record.items[1].get_name(), "o");
    }

    // ========== New convenience API tests ==========

    #[test]
    fn test_record_len_and_is_empty() {
        let mut record = DataRecord::default();
        assert!(record.is_empty());
        assert_eq!(record.len(), 0);

        record.append(FieldStorage::from_chars("a", "1"));
        assert!(!record.is_empty());
        assert_eq!(record.len(), 1);
    }

    #[test]
    fn test_from_vec_datafield() {
        let fields: Vec<DataField> = vec![
            DataField::from_chars("a", "1"),
            DataField::from_chars("b", "2"),
        ];
        let record = DataRecord::from(fields);
        assert_eq!(record.len(), 2);
        assert!(record.field("a").is_some());
        assert!(record.field("b").is_some());
    }

    #[test]
    fn test_from_single_datafield() {
        let field = DataField::from_chars("name", "Alice");
        let record = DataRecord::from(field);
        assert_eq!(record.len(), 1);
        assert!(record.field("name").is_some());
    }

    #[test]
    fn test_append_generic() {
        let mut record = DataRecord::default();

        // Append a DataField directly (auto-converts via Into<FieldStorage>)
        record.append(DataField::from_chars("a", "1"));

        // Append a FieldStorage directly (still works)
        record.append(FieldStorage::from_chars("b", "2"));

        // Append via Arc (auto-converts to Shared)
        let arc_field = Arc::new(DataField::from_chars("c", "3"));
        record.append(arc_field);

        assert_eq!(record.len(), 3);
        assert!(record.items[0].is_owned());
        assert!(record.items[1].is_owned());
        assert!(record.items[2].is_shared());
    }

    #[test]
    fn test_push_alias() {
        let mut record = DataRecord::default();
        record.push(DataField::from_chars("a", "1"));
        record.push(FieldStorage::from_chars("b", "2"));
        assert_eq!(record.len(), 2);
    }

    #[test]
    fn test_get_field_by_name() {
        let record = DataRecord::from(vec![
            DataField::from_chars("name", "Alice"),
            DataField::from_digit("age", 30),
        ]);

        let field = record.get_field("name");
        assert!(field.is_some());
        assert_eq!(field.unwrap().get_name(), "name");

        assert!(record.get_field("missing").is_none());
    }

    #[test]
    fn test_get_field_mut_by_name() {
        let mut record = DataRecord::from(vec![DataField::from_chars("name", "Alice")]);

        if let Some(field) = record.get_field_mut("name") {
            field.set_name("renamed");
        }
        assert_eq!(
            record.get_field("renamed").map(|f| f.get_name()),
            Some("renamed")
        );
    }

    #[test]
    fn test_field_mut() {
        let mut record = DataRecord::from(vec![DataField::from_chars("name", "Alice")]);

        let storage = record.field_mut("name");
        assert!(storage.is_some());
    }

    #[test]
    fn test_fields_iterator() {
        let record = DataRecord::from(vec![
            DataField::from_chars("a", "1"),
            DataField::from_chars("b", "2"),
            DataField::from_chars("c", "3"),
        ]);

        let names: Vec<&str> = record.fields().map(|f| f.get_name()).collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_fields_mut_iterator() {
        let mut record = DataRecord::from(vec![DataField::from_chars("a", "1")]);

        for field in record.fields_mut() {
            field.set_name("renamed");
        }
        assert_eq!(
            record.get_field("renamed").map(|f| f.get_name()),
            Some("renamed")
        );
    }
}
