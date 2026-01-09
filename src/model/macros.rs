#[macro_export]
macro_rules! value_match {
    ($obj:expr,$what :expr) => {
        match $obj {
            $crate::model::Value::Bool(x) => $what(x),
            $crate::model::Value::Chars(x) => $what(x),
            $crate::model::Value::Symbol(x) => $what(x.to_string()),
            $crate::model::Value::Digit(x) => $what(x),
            $crate::model::Value::Time(x) => $what(x),
            $crate::model::Value::Hex(x) => $what(x),
            $crate::model::Value::Float(x) => $what(x),
            $crate::model::Value::IpNet(x) => $what(x),
            $crate::model::Value::IpAddr(x) => $what(x),
            $crate::model::Value::Ignore(x) => $what(x),
            $crate::model::Value::Obj(x) => $what(x),
            $crate::model::Value::Array(x) => $what(x),
            $crate::model::Value::Domain(x) => $what(x),
            $crate::model::Value::Url(x) => $what(x),
            $crate::model::Value::Email(x) => $what(x),
            $crate::model::Value::IdCard(x) => $what(x),
            $crate::model::Value::MobilePhone(x) => $what(x),
        }
    };
    ($obj:expr,$what :expr,$a1:expr) => {
        match $obj {
            $crate::model::Value::Bool(x) => $what(x, $a1),
            $crate::model::Value::Chars(x) => $what(x, $a1),
            $crate::model::Value::Symbol(x) => $what(x.to_string(), $a1),
            $crate::model::Value::Digit(x) => $what(x, $a1),
            $crate::model::Value::Time(x) => $what(x, $a1),
            $crate::model::Value::Hex(x) => $what(x, $a1),
            $crate::model::Value::Float(x) => $what(x, $a1),
            $crate::model::Value::IpNet(x) => $what(x, $a1),
            $crate::model::Value::IpAddr(x) => $what(x, $a1),
            $crate::model::Value::Ignore(x) => $what(x, $a1),
            $crate::model::Value::Obj(x) => $what(x, $a1),
            $crate::model::Value::Array(x) => $what(x, $a1),
            $crate::model::Value::Domain(x) => $what(x, $a1),
            $crate::model::Value::Url(x) => $what(x, $a1),
            $crate::model::Value::Email(x) => $what(x, $a1),
            $crate::model::Value::IdCard(x) => $what(x, $a1),
            $crate::model::Value::MobilePhone(x) => $what(x, $a1),
        }
    };
    ($obj:expr,$what :expr,$a1:expr,$a2:expr) => {
        match $obj {
            $crate::model::Value::Bool(x) => $what(x, $a1, $a2),
            $crate::model::Value::Chars(x) => $what(x, $a1, $a2),
            $crate::model::Value::Symbol(x) => $what(x.to_string(), $a1, $a2),
            $crate::model::Value::Digit(x) => $what(x, $a1, $a2),
            $crate::model::Value::Time(x) => $what(x, $a1, $a2),
            $crate::model::Value::Hex(x) => $what(x, $a1, $a2),
            $crate::model::Value::Float(x) => $what(x, $a1, $a2),
            $crate::model::Value::IpNet(x) => $what(x, $a1, $a2),
            $crate::model::Value::IpAddr(x) => $what(x, $a1, $a2),
            $crate::model::Value::Ignore(x) => $what(x, $a1, $a2),
            $crate::model::Value::Obj(x) => $what(x, $a1, $a2),
            $crate::model::Value::Array(x) => $what(x, $a1, $a2),
            $crate::model::Value::Domain(x) => $what(x, $a1, $a2),
            $crate::model::Value::Url(x) => $what(x, $a1, $a2),
            $crate::model::Value::Email(x) => $what(x, $a1, $a2),
            $crate::model::Value::IdCard(x) => $what(x, $a1, $a2),
            $crate::model::Value::MobilePhone(x) => $what(x, $a1, $a2),
        }
    };
}

#[macro_export]
macro_rules! format_value {
    ($obj:expr,$what : ident ,$a1:expr) => {
        match $obj {
            $crate::model::Value::Bool(x) => $what(x).fmt($a1),
            $crate::model::Value::Chars(x) => $what(x).fmt($a1),
            $crate::model::Value::Symbol(x) => $what(x.to_string()).fmt($a1),
            $crate::model::Value::Digit(x) => $what(x).fmt($a1),
            $crate::model::Value::Time(x) => $what(x).fmt($a1),
            $crate::model::Value::Hex(x) => $what(x).fmt($a1),
            $crate::model::Value::Float(x) => $what(x).fmt($a1),
            $crate::model::Value::IpNet(x) => $what(x).fmt($a1),
            $crate::model::Value::IpAddr(x) => $what(x).fmt($a1),
            $crate::model::Value::Ignore(x) => $what(x).fmt($a1),
            $crate::model::Value::Obj(x) => $what(x).fmt($a1),
            $crate::model::Value::Array(x) => $what(x).fmt($a1),
            $crate::model::Value::Domain(x) => $what(x).fmt($a1),
            $crate::model::Value::Url(x) => $what(x).fmt($a1),
            $crate::model::Value::Email(x) => $what(x).fmt($a1),
            $crate::model::Value::IdCard(x) => $what(x).fmt($a1),
            $crate::model::Value::MobilePhone(x) => $what(x).fmt($a1),
        }
    };
}
