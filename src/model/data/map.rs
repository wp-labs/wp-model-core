use std::collections::BTreeMap;

use crate::model::Value;
use smol_str::SmolStr;

use super::field::Field;

pub type ObjectMap = BTreeMap<SmolStr, Field<Value>>;
