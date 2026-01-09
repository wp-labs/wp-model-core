use std::fmt::{Display, Formatter};

use chrono::NaiveDateTime;

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Eq)]
pub struct HexT(pub u128);

impl Display for HexT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}
pub type DigitValue = i64;
pub type FloatValue = f64;
pub type DateTimeValue = NaiveDateTime;
use serde::{Deserialize, Serialize};
