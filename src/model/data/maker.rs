use crate::model::{
    DataType, DateTimeValue, DomainT, EmailT, FNameStr, FValueStr, HexT, IdCardT, IgnoreT, Maker,
    MobilePhoneT, UrlValue, Value,
    types::value::{ObjectValue, SymbolValue},
};
use smol_str::SmolStr;
use std::net::IpAddr;

use super::Field;

impl<T> Field<T>
where
    T: Maker<bool>,
{
    pub fn from_bool<S: Into<FNameStr>>(name: S, val: bool) -> Self {
        Self::new(DataType::Bool, name.into(), T::make(val))
    }
}

impl<T> Field<T>
where
    T: Maker<FValueStr>,
{
    pub fn from_chars<N: Into<FNameStr>, V: Into<FValueStr>>(name: N, val: V) -> Self {
        Self::new(DataType::Chars, name.into(), T::make(val.into()))
    }
}
impl<T> Field<T>
where
    T: Maker<Value>,
{
    pub fn from_symbol<N: Into<FNameStr>, V: Into<SmolStr>>(name: N, val: V) -> Self {
        let value = SymbolValue::from(val.into());
        Self::new(DataType::Symbol, name.into(), T::make(value.into()))
    }
}

impl<T> Field<T>
where
    T: Maker<i64>,
{
    pub fn from_digit<S: Into<FNameStr>>(name: S, val: i64) -> Self {
        Self::new(DataType::Digit, name.into(), T::make(val))
    }
}
impl<T> Field<T>
where
    T: Maker<HexT>,
{
    pub fn from_hex<S: Into<FNameStr>>(name: S, val: HexT) -> Self {
        Self::new(DataType::Hex, name.into(), T::make(val))
    }
}
impl<T> Field<T>
where
    T: Maker<f64>,
{
    pub fn from_float<S: Into<FNameStr>>(name: S, val: f64) -> Self {
        Self::new(DataType::Float, name.into(), T::make(val))
    }
}
impl<T> Field<T>
where
    T: Maker<IpAddr>,
{
    pub fn from_ip<S: Into<FNameStr>>(name: S, ip: IpAddr) -> Self {
        Self::new(DataType::IP, name.into(), T::make(ip))
    }
}

impl<T> Field<T>
where
    T: Maker<DomainT>,
{
    pub fn from_domain<S: Into<FNameStr>, V: Into<SmolStr>>(name: S, domain: V) -> Self {
        Self::new(
            DataType::Domain,
            name.into(),
            T::make(DomainT(domain.into())),
        )
    }
}

impl<T> Field<T>
where
    T: Maker<UrlValue>,
{
    pub fn from_url<S: Into<FNameStr>, V: Into<SmolStr>>(name: S, url: V) -> Self {
        Self::new(DataType::Url, name.into(), T::make(UrlValue(url.into())))
    }
}

impl<T> Field<T>
where
    T: Maker<EmailT>,
{
    pub fn from_email<S: Into<FNameStr>, V: Into<SmolStr>>(name: S, email: V) -> Self {
        Self::new(DataType::Email, name.into(), T::make(EmailT(email.into())))
    }
}

impl<T> Field<T>
where
    T: Maker<IdCardT>,
{
    pub fn from_id_card<S: Into<FNameStr>, V: Into<SmolStr>>(name: S, id_card: V) -> Self {
        Self::new(
            DataType::IdCard,
            name.into(),
            T::make(IdCardT(id_card.into())),
        )
    }
}

impl<T> Field<T>
where
    T: Maker<MobilePhoneT>,
{
    pub fn from_mobile_phone<S: Into<FNameStr>, V: Into<SmolStr>>(
        name: S,
        mobile_phone: V,
    ) -> Self {
        Self::new(
            DataType::MobilePhone,
            name.into(),
            T::make(MobilePhoneT(mobile_phone.into())),
        )
    }
}

impl<T> Field<T>
where
    T: Maker<IgnoreT>,
{
    pub fn from_ignore<S: Into<FNameStr>>(name: S) -> Self {
        Self::new(DataType::Ignore, name.into(), T::make(IgnoreT::default()))
    }
}

impl<T> Field<T>
where
    T: Maker<DateTimeValue>,
{
    pub fn from_time<S: Into<FNameStr>>(name: S, val: DateTimeValue) -> Self {
        Self::new(DataType::Time, name.into(), T::make(val))
    }
}
impl<T> Field<T>
where
    T: Maker<Vec<Field<Value>>>,
{
    pub fn from_arr<S: Into<FNameStr>>(name: S, val: Vec<Field<Value>>) -> Self {
        if let Some(f) = val.first() {
            let meta = f.get_meta().to_string();
            Self::new(DataType::Array(meta), name.into(), T::make(val))
        } else {
            Self::new(DataType::Array("auto".into()), name.into(), T::make(val))
            //unreachable!("arr is empty");
        }
    }
}

impl<T> Field<T>
where
    T: Maker<ObjectValue>,
{
    pub fn from_obj<S: Into<FNameStr>>(name: S, val: ObjectValue) -> Self {
        Self::new(DataType::Obj, name.into(), T::make(val))
    }
}

impl Value {
    pub fn tag(&self) -> &str {
        match self {
            Value::Null => "Null",
            Value::Bool(_) => "Bool",
            Value::Chars(_) => "Chars",
            Value::Symbol(_) => "Symbol",
            Value::Digit(_) => "Digit",
            Value::Time(_) => "Time",
            Value::Hex(_) => "Hex",
            Value::Float(_) => "Float",
            Value::IpNet(_) => "IpNet",
            Value::IpAddr(_) => "IpAddr",
            Value::Ignore(_) => "Ignore",
            Value::Obj(_) => "Map",
            Value::Array(_) => "Array",
            Value::Domain(_) => "Domain",
            Value::Url(_) => "Url",
            Value::Email(_) => "Email",
            Value::IdCard(_) => "IdCard",
            Value::MobilePhone(_) => "MobilePhone",
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Value::Time(_)
            | Value::IpNet(_)
            | Value::IpAddr(_)
            | Value::Float(_)
            | Value::Digit(_)
            | Value::Bool(_)
            | Value::Hex(_) => false,
            Value::Domain(v) => v.0.is_empty(),
            Value::Url(v) => v.0.is_empty(),
            Value::Email(v) => v.0.is_empty(),
            Value::IdCard(v) => v.0.is_empty(),
            Value::MobilePhone(v) => v.0.is_empty(),
            Value::Chars(v) => v.is_empty(),
            Value::Obj(v) => v.is_empty(),
            Value::Array(v) => v.is_empty(),
            Value::Symbol(v) => v.is_empty(),
            Value::Ignore(_) => true,
            Value::Null => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DataField, FValueStr};
    use chrono::NaiveDateTime;
    use smol_str::SmolStr;
    use std::net::{IpAddr, Ipv4Addr};

    // ========== Field factory method tests ==========

    #[test]
    fn test_field_from_bool() {
        let field: DataField = Field::from_bool("is_active", true);
        assert_eq!(field.get_name(), "is_active");
        assert_eq!(field.meta, DataType::Bool);
        assert_eq!(field.value, Value::Bool(true));
    }

    #[test]
    fn test_field_from_chars() {
        let field: DataField = Field::from_chars("message", "hello");
        assert_eq!(field.get_name(), "message");
        assert_eq!(field.meta, DataType::Chars);
        assert_eq!(field.value, Value::Chars(FValueStr::from("hello")));
    }

    #[test]
    fn test_field_from_symbol() {
        let field: DataField = Field::from_symbol("status", "OK");
        assert_eq!(field.get_name(), "status");
        assert_eq!(field.meta, DataType::Symbol);
        assert_eq!(field.value, Value::Symbol(SmolStr::from("OK")));
    }

    #[test]
    fn test_field_from_digit() {
        let field: DataField = Field::from_digit("count", 42);
        assert_eq!(field.get_name(), "count");
        assert_eq!(field.meta, DataType::Digit);
        assert_eq!(field.value, Value::Digit(42));
    }

    #[test]
    fn test_field_from_float() {
        let field: DataField = Field::from_float("ratio", 2.14);
        assert_eq!(field.get_name(), "ratio");
        assert_eq!(field.meta, DataType::Float);
        assert_eq!(field.value, Value::Float(2.14));
    }

    #[test]
    fn test_field_from_hex() {
        let field: DataField = Field::from_hex("color", HexT(0xFF00FF));
        assert_eq!(field.get_name(), "color");
        assert_eq!(field.meta, DataType::Hex);
        assert_eq!(field.value, Value::Hex(HexT(0xFF00FF)));
    }

    #[test]
    fn test_field_from_ip() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
        let field: DataField = Field::from_ip("src_ip", ip);
        assert_eq!(field.get_name(), "src_ip");
        assert_eq!(field.meta, DataType::IP);
        assert_eq!(field.value, Value::IpAddr(ip));
    }

    #[test]
    fn test_field_from_domain() {
        let field: DataField = Field::from_domain("host", "example.com");
        assert_eq!(field.get_name(), "host");
        assert_eq!(field.meta, DataType::Domain);
        assert_eq!(field.value, Value::Domain(DomainT("example.com".into())));
    }

    #[test]
    fn test_field_from_url() {
        let field: DataField = Field::from_url("link", "https://example.com");
        assert_eq!(field.get_name(), "link");
        assert_eq!(field.meta, DataType::Url);
        assert_eq!(
            field.value,
            Value::Url(UrlValue("https://example.com".into()))
        );
    }

    #[test]
    fn test_field_from_email() {
        let field: DataField = Field::from_email("contact", "test@example.com");
        assert_eq!(field.get_name(), "contact");
        assert_eq!(field.meta, DataType::Email);
        assert_eq!(field.value, Value::Email(EmailT("test@example.com".into())));
    }

    #[test]
    fn test_field_from_id_card() {
        let field: DataField = Field::from_id_card("id", "123456789");
        assert_eq!(field.get_name(), "id");
        assert_eq!(field.meta, DataType::IdCard);
        assert_eq!(field.value, Value::IdCard(IdCardT("123456789".into())));
    }

    #[test]
    fn test_field_from_mobile_phone() {
        let field: DataField = Field::from_mobile_phone("phone", "13800138000");
        assert_eq!(field.get_name(), "phone");
        assert_eq!(field.meta, DataType::MobilePhone);
        assert_eq!(
            field.value,
            Value::MobilePhone(MobilePhoneT("13800138000".into()))
        );
    }

    #[test]
    fn test_field_from_ignore() {
        let field: DataField = Field::from_ignore("unused");
        assert_eq!(field.get_name(), "unused");
        assert_eq!(field.meta, DataType::Ignore);
        assert_eq!(field.value, Value::Ignore(IgnoreT::default()));
    }

    #[test]
    fn test_field_from_time() {
        let dt = NaiveDateTime::parse_from_str("2024-01-15 10:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let field: DataField = Field::from_time("timestamp", dt);
        assert_eq!(field.get_name(), "timestamp");
        assert_eq!(field.meta, DataType::Time);
        assert_eq!(field.value, Value::Time(dt));
    }

    #[test]
    fn test_field_from_arr_with_elements() {
        let arr = vec![Field::from_digit("item", 1), Field::from_digit("item", 2)];
        let field: DataField = Field::from_arr("numbers", arr);
        assert_eq!(field.get_name(), "numbers");
        assert_eq!(field.meta, DataType::Array("digit".into()));
    }

    #[test]
    fn test_field_from_arr_empty() {
        let arr: Vec<DataField> = vec![];
        let field: DataField = Field::from_arr("empty", arr);
        assert_eq!(field.get_name(), "empty");
        assert_eq!(field.meta, DataType::Array("auto".into()));
    }

    #[test]
    fn test_field_from_obj() {
        let obj = ObjectValue::new();
        let field: DataField = Field::from_obj("data", obj.clone());
        assert_eq!(field.get_name(), "data");
        assert_eq!(field.meta, DataType::Obj);
        assert_eq!(field.value, Value::Obj(obj));
    }

    // ========== Value::tag() tests ==========

    #[test]
    fn test_value_tag() {
        assert_eq!(Value::Null.tag(), "Null");
        assert_eq!(Value::Bool(true).tag(), "Bool");
        assert_eq!(Value::Chars(FValueStr::from("x")).tag(), "Chars");
        assert_eq!(Value::Symbol(SmolStr::from("x")).tag(), "Symbol");
        assert_eq!(Value::Digit(1).tag(), "Digit");
        assert_eq!(Value::Float(1.0).tag(), "Float");
        assert_eq!(Value::Hex(HexT(0)).tag(), "Hex");
        assert_eq!(Value::Ignore(IgnoreT::default()).tag(), "Ignore");
        assert_eq!(Value::Obj(ObjectValue::new()).tag(), "Map");
        assert_eq!(Value::Array(vec![]).tag(), "Array");
        assert_eq!(Value::Domain(DomainT("x".into())).tag(), "Domain");
        assert_eq!(Value::Url(UrlValue("x".into())).tag(), "Url");
        assert_eq!(Value::Email(EmailT("x".into())).tag(), "Email");
        assert_eq!(Value::IdCard(IdCardT("x".into())).tag(), "IdCard");
        assert_eq!(
            Value::MobilePhone(MobilePhoneT("x".into())).tag(),
            "MobilePhone"
        );
    }

    #[test]
    fn test_value_tag_ip() {
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(Value::IpAddr(ip).tag(), "IpAddr");
    }

    // ========== Value::is_empty() tests ==========

    #[test]
    fn test_is_empty_always_false() {
        // These types are never considered empty
        assert!(!Value::Bool(false).is_empty());
        assert!(!Value::Digit(0).is_empty());
        assert!(!Value::Float(0.0).is_empty());
        assert!(!Value::Hex(HexT(0)).is_empty());
    }

    #[test]
    fn test_is_empty_always_true() {
        assert!(Value::Null.is_empty());
        assert!(Value::Ignore(IgnoreT::default()).is_empty());
    }

    #[test]
    fn test_is_empty_string_types() {
        // Empty strings
        assert!(Value::Chars(FValueStr::from("")).is_empty());
        assert!(Value::Symbol(SmolStr::from("")).is_empty());
        assert!(Value::Domain(DomainT("".into())).is_empty());
        assert!(Value::Url(UrlValue("".into())).is_empty());
        assert!(Value::Email(EmailT("".into())).is_empty());
        assert!(Value::IdCard(IdCardT("".into())).is_empty());
        assert!(Value::MobilePhone(MobilePhoneT("".into())).is_empty());

        // Non-empty strings
        assert!(!Value::Chars(FValueStr::from("x")).is_empty());
        assert!(!Value::Symbol(SmolStr::from("x")).is_empty());
        assert!(!Value::Domain(DomainT("x".into())).is_empty());
        assert!(!Value::Url(UrlValue("x".into())).is_empty());
        assert!(!Value::Email(EmailT("x".into())).is_empty());
        assert!(!Value::IdCard(IdCardT("x".into())).is_empty());
        assert!(!Value::MobilePhone(MobilePhoneT("x".into())).is_empty());
    }

    #[test]
    fn test_is_empty_collections() {
        // Empty
        assert!(Value::Array(vec![]).is_empty());
        assert!(Value::Obj(ObjectValue::new()).is_empty());

        // Non-empty
        let arr = vec![Field::from_digit("x", 1)];
        assert!(!Value::Array(arr).is_empty());
    }
}
