use std::fmt::Formatter;

pub struct MetaFmt<T>(pub T);

pub struct FunFmt<T>(pub T);
pub struct KVFmt<T>(pub T);

pub struct CSVFmt<T>(pub T);

//Nol
#[derive(Debug)]
pub struct NormalFmt<T>(pub T);
#[derive(Debug)]
pub struct RawFmt<T>(pub T);

pub struct JsonFmt<T>(pub T);

pub struct SqlFmt<T>(pub T);

pub struct ProtoFmt<T>(pub T);

pub trait LevelFormatAble {
    fn level_fmt(&self, f: &mut Formatter<'_>, level: usize) -> std::fmt::Result;
}

impl std::fmt::Display for MetaFmt<&crate::model::DataRecord> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::model::DataType;
        write!(f, "\nline:\n(")?;
        for o in self.0.items.iter() {
            if *o.get_meta() != DataType::Ignore {
                write!(f, "{},", String::from(o.get_meta()))?;
            }
        }
        write!(f, ")")
    }
}
