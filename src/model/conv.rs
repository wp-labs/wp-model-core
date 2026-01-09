use crate::model::{DataField, DataRecord, SharedRecord};

/*
impl From<DataRecord> for SharedRecord {
    fn from(value: DataRecord) -> Self {
        let mut items = Vec::with_capacity(value.items.len());
        for tdo in value.items {
            items.push(SharedField::from(tdo))
        }
        Self { items }
    }
}

impl From<SharedRecord> for DataRecord {
    fn from(value: SharedRecord) -> Self {
        let mut items = Vec::with_capacity(value.items.len());
        for tdo in value.items {
            items.push(DataField::from(tdo))
        }
        Self { items }
    }
}
pub fn to_value_field_vec(value: Vec<SharedField>) -> Vec<DataField> {
    let mut items = Vec::with_capacity(value.len());
    for tdo in value {
        items.push(DataField::from(tdo))
    }
    items
}

pub fn to_shared_field_vec(value: Vec<DataField>) -> Vec<SharedField> {
    let mut items = Vec::with_capacity(value.len());
    for tdo in value {
        items.push(SharedField::from(tdo))
    }
    items
}
*/
