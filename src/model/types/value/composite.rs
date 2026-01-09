use crate::model::DataField;

use super::Value;
use smol_str::SmolStr;
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ObjectValue(pub BTreeMap<SmolStr, DataField>);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayValue(pub Vec<Value>);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IgnoreValue;

#[derive(PartialEq, Default, Clone, Debug, Serialize, Deserialize)]
pub struct IgnoreT {}

impl ObjectValue {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn new() -> Self {
        ObjectValue(BTreeMap::new())
    }

    pub fn insert<S: Into<SmolStr>>(&mut self, key: S, value: DataField) {
        self.0.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&DataField> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut DataField> {
        self.0.get_mut(key)
    }
}

impl Display for ObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for ObjectValue {
    type Target = BTreeMap<SmolStr, DataField>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ObjectValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
use serde::{Deserialize, Serialize};
