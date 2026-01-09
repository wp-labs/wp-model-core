use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, PartialEq, Clone, Hash, Eq, Serialize, Deserialize, Default)]
pub enum DataType {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "chars")]
    Chars,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "peek_symbol")]
    PeekSymbol,
    #[serde(rename = "digit")]
    Digit,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "ignore")]
    Ignore,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "time_iso")]
    TimeISO,
    #[serde(rename = "time_3339")]
    TimeRFC3339,
    #[serde(rename = "time_2822")]
    TimeRFC2822,
    #[serde(rename = "time_timestamp")]
    TimeTIMESTAMP,
    #[serde(rename = "time_clf")]
    TimeCLF,
    //#[serde(rename = "time_timestamp_ms)")]
    //TimeTimestampMs,
    //#[serde(rename = "time_timestamp_us)")]
    //TimeTimestampUs,
    #[serde(rename = "ip")]
    IP,
    #[serde(rename = "ip_net")]
    IpNet,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "port")]
    Port,
    #[serde(rename = "sn")]
    SN,
    #[serde(rename = "hex")]
    Hex,
    #[serde(rename = "base64")]
    Base64,
    #[serde(rename = "kv")]
    KV,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "exact_json")]
    ExactJson,
    #[serde(rename = "http_request")]
    HttpRequest,
    #[serde(rename = "http_status")]
    HttpStatus,
    #[serde(rename = "http_agent")]
    HttpAgent,
    #[serde(rename = "http_method")]
    HttpMethod,
    #[serde(rename = "url")]
    Url,
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "proto-text")]
    ProtoText,
    #[serde(rename = "obj")]
    Obj,
    #[serde(rename = "array")]
    Array(String),
    #[serde(rename = "id_card")]
    IdCard,
    #[serde(rename = "mobile_phone")]
    MobilePhone,
}

pub const CHARS: &str = "chars";
pub const DIGIT: &str = "digit";
pub const BOOL: &str = "bool";
pub const FLOAT: &str = "float";
pub const TIME: &str = "time";
pub const TIME_ISO: &str = "time_iso";
pub const TIME_RFC3339: &str = "time_3339";
pub const TIME_RFC2822: &str = "time_2822";
pub const TIME_TIMESTAMP: &str = "time_timestamp";
pub const TIME_CLF: &str = "time_clf";
pub const IP: &str = "ip";
pub const IP_NET: &str = "ip_net";
pub const DOMAIN: &str = "domain";
pub const EMAIL: &str = "email";
pub const SN: &str = "sn";
pub const PORT: &str = "port";
pub const HEXDIGIT: &str = "hex";
pub const KV: &str = "kv";
pub const JSON: &str = "json";
pub const EXACT_JSON: &str = "exact_json";
pub const HTTP_REQUEST: &str = "http/request";
pub const HTTP_STATUS: &str = "http/status";
pub const HTTP_AGENT: &str = "http/agent";
pub const HTTP_METHOD: &str = "http/method";
pub const URL: &str = "url";
pub const IGNORE: &str = "_";
pub const AUTO: &str = "auto";
pub const BASE64: &str = "base64";
pub const PROTO_TEXT: &str = "proto_text";

pub const SYMBOL: &str = "symbol";
pub const PEEK_SYMBOL: &str = "peek_symbol";
pub const OBJ: &str = "obj";
pub const ARRAY: &str = "array";
pub const ID_CARD: &str = "id_card";
pub const MOBILE_PHONE: &str = "mobile_phone";

#[derive(Error, Debug)]
pub enum MetaErr {
    #[error("meta not support : {0}")]
    UnSupport(String),
}

impl DataType {
    pub fn from(value: &str) -> Result<Self, MetaErr> {
        match value {
            // Aliases (namespaced or clearer variants)
            // De-facto standard in HTTP access logs: Common Log Format (CLF)
            // Used by Apache httpd and Nginx; timestamp like: dd/Mon/yyyy:HH:MM:SS ±ZZZZ
            "time/apache" | "time/clf" | "time/httpd" | "time/nginx" => Ok(DataType::TimeCLF),
            "time/timestamp" => Ok(DataType::TimeTIMESTAMP),
            "time/epoch" => Ok(DataType::TimeTIMESTAMP),
            "time/rfc3339" => Ok(DataType::TimeRFC3339),
            "time/rfc2822" => Ok(DataType::TimeRFC2822),
            "json/strict" => Ok(DataType::ExactJson),
            "proto/text" => Ok(DataType::ProtoText),
            "http/user_agent" => Ok(DataType::HttpAgent),
            "object" => Ok(DataType::Obj),
            "symbol/peek" => Ok(DataType::PeekSymbol),
            BOOL => Ok(DataType::Bool),
            TIME => Ok(DataType::Time),
            TIME_ISO => Ok(DataType::TimeISO),
            TIME_RFC3339 => Ok(DataType::TimeRFC3339),
            TIME_RFC2822 => Ok(DataType::TimeRFC2822),
            TIME_TIMESTAMP => Ok(DataType::TimeTIMESTAMP),
            IP => Ok(DataType::IP),
            IP_NET => Ok(DataType::IpNet),
            DOMAIN => Ok(DataType::Domain),
            EMAIL => Ok(DataType::Email),
            SN => Ok(DataType::SN),
            PORT => Ok(DataType::Port),
            HEXDIGIT => Ok(DataType::Hex),
            KV => Ok(DataType::KV),
            JSON => Ok(DataType::Json),
            EXACT_JSON => Ok(DataType::ExactJson),
            HTTP_REQUEST | "http_request" => Ok(DataType::HttpRequest),
            HTTP_STATUS | "http_status" => Ok(DataType::HttpStatus),
            HTTP_AGENT | "http_agent" => Ok(DataType::HttpAgent),
            HTTP_METHOD | "http_method" => Ok(DataType::HttpMethod),
            URL => Ok(DataType::Url),
            CHARS => Ok(DataType::Chars),
            SYMBOL => Ok(DataType::Symbol),
            PEEK_SYMBOL => Ok(DataType::PeekSymbol),
            DIGIT => Ok(DataType::Digit),
            FLOAT => Ok(DataType::Float),
            AUTO => Ok(DataType::Auto),
            BASE64 => Ok(DataType::Base64),
            IGNORE => Ok(DataType::Ignore),
            PROTO_TEXT => Ok(DataType::ProtoText),
            OBJ => Ok(DataType::Obj),
            ID_CARD => Ok(DataType::IdCard),
            MOBILE_PHONE => Ok(DataType::MobilePhone),
            //ARRAY => Ok(Meta::Array),
            _ => Self::to_arr(value), //Err(MetaErr::UnSupport(format!("unknown meta: {}", value))),
        }
    }
    pub fn to_arr(value: &str) -> Result<Self, MetaErr> {
        if let Some(rest) = value.strip_prefix(ARRAY) {
            if rest.is_empty() {
                return Ok(DataType::Array("auto".into()));
            }
            if let Some(sub) = rest.strip_prefix('/') {
                if sub.is_empty() {
                    return Ok(DataType::Array("auto".into()));
                }
                return Ok(DataType::Array(sub.to_string()));
            }
            return Err(MetaErr::UnSupport(format!(
                "unknown meta: {} (array missing subtype)",
                value
            )));
        }
        Err(MetaErr::UnSupport(format!("unknown meta: {}", value)))
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = String::from(self);
        write!(f, "{}", string)
    }
}

impl DataType {
    pub fn static_name(&self) -> &'static str {
        match self {
            DataType::Bool => BOOL,
            DataType::Time => TIME,
            DataType::TimeISO => TIME_ISO,
            DataType::TimeRFC3339 => TIME_RFC3339,
            DataType::TimeRFC2822 => TIME_RFC2822,
            DataType::TimeTIMESTAMP => TIME_TIMESTAMP,
            DataType::TimeCLF => TIME_CLF,
            DataType::IP => IP,
            DataType::IpNet => IP_NET,
            DataType::Domain => DOMAIN,
            DataType::Email => EMAIL,
            DataType::SN => SN,
            DataType::Port => PORT,
            DataType::Hex => HEXDIGIT,
            DataType::KV => KV,
            DataType::Json => JSON,
            DataType::ExactJson => EXACT_JSON,
            DataType::Chars => CHARS,
            DataType::Symbol => SYMBOL,
            DataType::PeekSymbol => PEEK_SYMBOL,
            DataType::Digit => DIGIT,
            DataType::Float => FLOAT,
            DataType::HttpRequest => HTTP_REQUEST,
            DataType::HttpStatus => HTTP_STATUS,
            DataType::HttpAgent => HTTP_AGENT,
            DataType::HttpMethod => HTTP_METHOD,
            DataType::Url => URL,
            DataType::Ignore => IGNORE,
            DataType::Auto => AUTO,
            DataType::Base64 => BASE64,
            DataType::ProtoText => PROTO_TEXT,
            DataType::Obj => OBJ,
            DataType::Array(_) => ARRAY,
            DataType::IdCard => ID_CARD,
            DataType::MobilePhone => MOBILE_PHONE,
        }
    }
    pub fn parse_patten_first(&self) -> bool {
        !matches!(
            self,
            DataType::Chars | DataType::Ignore | DataType::SN | DataType::Auto
        )
    }
}
impl From<&DataType> for String {
    fn from(value: &DataType) -> Self {
        if let DataType::Array(x) = value {
            return format!("{}/{}", ARRAY, x);
        }
        value.static_name().to_string()
    }
}
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_primitive_types() {
        assert_eq!(DataType::from("bool").unwrap(), DataType::Bool);
        assert_eq!(DataType::from("chars").unwrap(), DataType::Chars);
        assert_eq!(DataType::from("digit").unwrap(), DataType::Digit);
        assert_eq!(DataType::from("float").unwrap(), DataType::Float);
        assert_eq!(DataType::from("symbol").unwrap(), DataType::Symbol);
        assert_eq!(DataType::from("auto").unwrap(), DataType::Auto);
        assert_eq!(DataType::from("_").unwrap(), DataType::Ignore);
    }

    #[test]
    fn test_from_time_types() {
        assert_eq!(DataType::from("time").unwrap(), DataType::Time);
        assert_eq!(DataType::from("time_iso").unwrap(), DataType::TimeISO);
        assert_eq!(DataType::from("time_3339").unwrap(), DataType::TimeRFC3339);
        assert_eq!(DataType::from("time_2822").unwrap(), DataType::TimeRFC2822);
        assert_eq!(
            DataType::from("time_timestamp").unwrap(),
            DataType::TimeTIMESTAMP
        );
        // time_clf uses aliases like time/clf, time/apache, etc.
    }

    #[test]
    fn test_from_time_aliases() {
        // CLF aliases
        assert_eq!(DataType::from("time/apache").unwrap(), DataType::TimeCLF);
        assert_eq!(DataType::from("time/clf").unwrap(), DataType::TimeCLF);
        assert_eq!(DataType::from("time/httpd").unwrap(), DataType::TimeCLF);
        assert_eq!(DataType::from("time/nginx").unwrap(), DataType::TimeCLF);
        // Other aliases
        assert_eq!(
            DataType::from("time/timestamp").unwrap(),
            DataType::TimeTIMESTAMP
        );
        assert_eq!(
            DataType::from("time/epoch").unwrap(),
            DataType::TimeTIMESTAMP
        );
        assert_eq!(
            DataType::from("time/rfc3339").unwrap(),
            DataType::TimeRFC3339
        );
        assert_eq!(
            DataType::from("time/rfc2822").unwrap(),
            DataType::TimeRFC2822
        );
    }

    #[test]
    fn test_from_network_types() {
        assert_eq!(DataType::from("ip").unwrap(), DataType::IP);
        assert_eq!(DataType::from("ip_net").unwrap(), DataType::IpNet);
        assert_eq!(DataType::from("domain").unwrap(), DataType::Domain);
        assert_eq!(DataType::from("email").unwrap(), DataType::Email);
        assert_eq!(DataType::from("port").unwrap(), DataType::Port);
        assert_eq!(DataType::from("url").unwrap(), DataType::Url);
    }

    #[test]
    fn test_from_http_types() {
        assert_eq!(
            DataType::from("http/request").unwrap(),
            DataType::HttpRequest
        );
        assert_eq!(
            DataType::from("http_request").unwrap(),
            DataType::HttpRequest
        );
        assert_eq!(DataType::from("http/status").unwrap(), DataType::HttpStatus);
        assert_eq!(DataType::from("http_status").unwrap(), DataType::HttpStatus);
        assert_eq!(DataType::from("http/agent").unwrap(), DataType::HttpAgent);
        assert_eq!(
            DataType::from("http/user_agent").unwrap(),
            DataType::HttpAgent
        );
        assert_eq!(DataType::from("http/method").unwrap(), DataType::HttpMethod);
    }

    #[test]
    fn test_from_special_types() {
        assert_eq!(DataType::from("hex").unwrap(), DataType::Hex);
        assert_eq!(DataType::from("base64").unwrap(), DataType::Base64);
        assert_eq!(DataType::from("kv").unwrap(), DataType::KV);
        assert_eq!(DataType::from("json").unwrap(), DataType::Json);
        assert_eq!(DataType::from("exact_json").unwrap(), DataType::ExactJson);
        assert_eq!(DataType::from("json/strict").unwrap(), DataType::ExactJson);
        assert_eq!(DataType::from("proto_text").unwrap(), DataType::ProtoText);
        assert_eq!(DataType::from("proto/text").unwrap(), DataType::ProtoText);
        assert_eq!(DataType::from("obj").unwrap(), DataType::Obj);
        assert_eq!(DataType::from("object").unwrap(), DataType::Obj);
        assert_eq!(DataType::from("id_card").unwrap(), DataType::IdCard);
        assert_eq!(
            DataType::from("mobile_phone").unwrap(),
            DataType::MobilePhone
        );
    }

    #[test]
    fn test_to_arr_parsing() {
        // "array" alone -> Array("auto")
        assert_eq!(
            DataType::to_arr("array").unwrap(),
            DataType::Array("auto".into())
        );
        // "array/" -> Array("auto")
        assert_eq!(
            DataType::to_arr("array/").unwrap(),
            DataType::Array("auto".into())
        );
        // "array/digit" -> Array("digit")
        assert_eq!(
            DataType::to_arr("array/digit").unwrap(),
            DataType::Array("digit".into())
        );
        assert_eq!(
            DataType::to_arr("array/chars").unwrap(),
            DataType::Array("chars".into())
        );
    }

    #[test]
    fn test_from_array_types() {
        assert_eq!(
            DataType::from("array").unwrap(),
            DataType::Array("auto".into())
        );
        assert_eq!(
            DataType::from("array/ip").unwrap(),
            DataType::Array("ip".into())
        );
    }

    #[test]
    fn test_from_unsupported_type() {
        let result = DataType::from("unknown_type");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("unknown meta"));
    }

    #[test]
    fn test_to_arr_invalid() {
        // Not starting with "array"
        assert!(DataType::to_arr("notarray").is_err());
        // "arrayfoo" (no slash after array)
        assert!(DataType::to_arr("arrayfoo").is_err());
    }

    #[test]
    fn test_static_name() {
        assert_eq!(DataType::Bool.static_name(), "bool");
        assert_eq!(DataType::Chars.static_name(), "chars");
        assert_eq!(DataType::Digit.static_name(), "digit");
        assert_eq!(DataType::Float.static_name(), "float");
        assert_eq!(DataType::Time.static_name(), "time");
        assert_eq!(DataType::IP.static_name(), "ip");
        assert_eq!(DataType::Ignore.static_name(), "_");
        assert_eq!(DataType::Array("digit".into()).static_name(), "array");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DataType::Bool), "bool");
        assert_eq!(format!("{}", DataType::Chars), "chars");
        assert_eq!(
            format!("{}", DataType::Array("digit".into())),
            "array/digit"
        );
    }

    #[test]
    fn test_string_from_datatype() {
        assert_eq!(String::from(&DataType::Bool), "bool");
        assert_eq!(String::from(&DataType::IP), "ip");
        assert_eq!(
            String::from(&DataType::Array("chars".into())),
            "array/chars"
        );
    }

    #[test]
    fn test_parse_pattern_first() {
        // Should return false
        assert!(!DataType::Chars.parse_patten_first());
        assert!(!DataType::Ignore.parse_patten_first());
        assert!(!DataType::SN.parse_patten_first());
        assert!(!DataType::Auto.parse_patten_first());

        // Should return true
        assert!(DataType::Bool.parse_patten_first());
        assert!(DataType::Digit.parse_patten_first());
        assert!(DataType::IP.parse_patten_first());
        assert!(DataType::Time.parse_patten_first());
    }

    #[test]
    fn test_default() {
        assert_eq!(DataType::default(), DataType::Auto);
    }

    #[test]
    fn test_serde_roundtrip() {
        let types = vec![
            DataType::Bool,
            DataType::Chars,
            DataType::Digit,
            DataType::Array("ip".into()),
        ];
        for dt in types {
            let json = serde_json::to_string(&dt).unwrap();
            let parsed: DataType = serde_json::from_str(&json).unwrap();
            assert_eq!(dt, parsed);
        }
    }
}
