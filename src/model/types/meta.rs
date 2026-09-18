use std::fmt::{Display, Formatter};
use thiserror::Error;

/// `DataType::Array` 的子类型：**自由字符串**（不校验是否为已知类型名），但**前后空白已规范化**。
///
/// 载荷私有 ⇒ 构造不出脏值：外部只能经 [`ArraySubtype::new`] / `From<&str>` / `From<String>` /
/// serde 反序列化拿到它，四条路都会 trim（trim 后为空 → `"auto"`，与 `to_arr("array")` /
/// `to_arr("array/")` 同口径）。
///
/// 为何值得单开一个类型：`DataType` 派生 `PartialEq`/`Eq`/`Hash`，若子类型能带空白，
/// `Array("int ")` 与 `Array("int")` 就是**两个不同类型** —— 用元类型做等值判断、当
/// `HashMap` key 查类型映射时会**静默不匹配**（不报错）。做成不变量后这类坑在类型层面消失。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArraySubtype(String);

impl ArraySubtype {
    /// 规范化构造：trim 前后空白；trim 后为空 → `"auto"`。
    pub fn new(sub: &str) -> Self {
        let sub = sub.trim();
        Self(if sub.is_empty() {
            "auto".to_string()
        } else {
            sub.to_string()
        })
    }

    /// 规范化后的子类型（只削前后空白，**内部空白保留**）。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for ArraySubtype {
    fn from(sub: &str) -> Self {
        Self::new(sub)
    }
}

impl From<String> for ArraySubtype {
    fn from(sub: String) -> Self {
        Self::new(&sub)
    }
}

impl std::ops::Deref for ArraySubtype {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl Display for ArraySubtype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 与 `&str` 直接比较，便于 `if sub == "int"` 这类写法。
impl PartialEq<&str> for ArraySubtype {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl Serialize for ArraySubtype {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ArraySubtype {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // wire 输入同样过规范化入口，否则 `{"array":"int "}` 会绕过不变量。
        Ok(Self::new(&String::deserialize(deserializer)?))
    }
}

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
    #[serde(rename = "int")]
    Int,
    /// 任意精度无符号整数（配合 `Value::BigUint`，十进制字符串存储/绑定）
    #[serde(rename = "bigint")]
    BigInt,
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
    #[serde(rename = "kvarr")]
    KvArr,
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
    Array(ArraySubtype),
    #[serde(rename = "id_card")]
    IdCard,
    #[serde(rename = "mobile_phone")]
    MobilePhone,
}

pub const CHARS: &str = "chars";
pub const INT: &str = "int";
pub const BIGINT: &str = "bigint";
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
pub const KVARR: &str = "kvarr";
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
        // 类型名通常从文本读入（schema / DDL / 配置），先规范化**外层**空白：
        // `" int "` → `Int`。大小写仍然是严格的（`"Int"` 不被接受，见常量表）。
        let value = value.trim();
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
            KVARR => Ok(DataType::KvArr),
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
            INT => Ok(DataType::Int),
            BIGINT => Ok(DataType::BigInt),
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
        // 外部可直接调本函数，所以外层空白也在这里规范化（`from` 已 trim 过一遍）。
        let value = value.trim();
        if let Some(rest) = value.strip_prefix(ARRAY) {
            // 子类型是**自由字符串**（`Array(String)`），不校验是否为已知类型；但**前后**空白
            // 必须规范化：`Array("int ")` 与 `Array("int")` 是两个不同的类型，下游按子类型
            // 字符串比较时会**静默不匹配**。
            //
            // 边界（刻意如此）：只削前后空白，**内部空白保留**（`"array/i nt"` → `Array("i nt")`）；
            // 规范化只落在本函数与 `from`（parse 侧）——serde 反序列化（`{"array":"int "}`）
            // 与 `Display`/`String::from` 产出不经此路，两侧不对称由测试钉住。
            let rest = rest.trim();
            if rest.is_empty() {
                return Ok(DataType::Array("auto".into()));
            }
            if let Some(sub) = rest.strip_prefix('/') {
                let sub = sub.trim();
                if sub.is_empty() {
                    return Ok(DataType::Array("auto".into()));
                }
                return Ok(DataType::Array(sub.into()));
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
            DataType::KvArr => KVARR,
            DataType::KV => KV,
            DataType::Json => JSON,
            DataType::ExactJson => EXACT_JSON,
            DataType::Chars => CHARS,
            DataType::Symbol => SYMBOL,
            DataType::PeekSymbol => PEEK_SYMBOL,
            DataType::Int => INT,
            DataType::BigInt => BIGINT,
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
        assert_eq!(DataType::from("int").unwrap(), DataType::Int);
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
        // "array/int" -> Array("int")
        assert_eq!(
            DataType::to_arr("array/int").unwrap(),
            DataType::Array("int".into())
        );
        assert_eq!(
            DataType::to_arr("array/chars").unwrap(),
            DataType::Array("chars".into())
        );
    }

    /// `Array(subtype)` 的子类型是自由字符串：**容忍**任意（含已退休的 `digit`），
    /// 但**空白必须规范化**（否则 `Array("int ")` 与 `Array("int")` 静默不等）。
    #[test]
    fn test_to_arr_subtype_is_free_form_but_whitespace_normalized() {
        // 自由字符串：不校验已知类型名，旧名 `digit` 也照样收（0.10.0 的既有决定）
        assert_eq!(
            DataType::from("array/digit").unwrap(),
            DataType::Array("digit".into())
        );
        assert_eq!(
            DataType::from("array/whatever").unwrap(),
            DataType::Array("whatever".into())
        );
        // 空白规范化：三种写法都得 `Array("int")`
        for raw in ["array/int ", "array/ int", "array/  int  "] {
            assert_eq!(
                DataType::from(raw).unwrap(),
                DataType::Array("int".into()),
                "{raw:?} 应规范化到 Array(\"int\")"
            );
        }
        // 只有空白（或全空白）的子类型 → auto
        assert_eq!(
            DataType::from("array/   ").unwrap(),
            DataType::Array("auto".into())
        );
        // 不变量（`ArraySubtype` 载荷私有 + 构造必规范化）：两种写法**就是同一个类型**，
        // 不可能再出现“看起来一样、实则不相等”的静默不匹配。
        assert_eq!(
            DataType::Array("int ".into()),
            DataType::Array("int".into())
        );
        assert_eq!(
            DataType::Array("int ".into()).to_string(),
            "array/int",
            "带空格的输入也产出规范化的显示形态"
        );
        // 规范化后的子类型与原样写法**同类型**（键/比较/Display 一致）
        assert_eq!(
            DataType::from("array/int ").unwrap(),
            DataType::from("array/int").unwrap()
        );
        assert_eq!(format!("{}", DataType::Array("int".into())), "array/int");
    }

    /// 空白规范化（外层名字）：类型名多从文本读入（schema / DDL / 配置），
    /// 前后空白不应造成“看起来一样、实则解析失败 / 不同类型”的陷阱。
    ///
    /// 大小写**仍是严格的**：`"Int"` / `"ARRAY/int"` 依旧被拒（不在本次变更范围）。
    #[test]
    fn test_from_trims_outer_whitespace_and_stays_case_sensitive() {
        assert_eq!(DataType::from(" int ").unwrap(), DataType::Int);
        assert_eq!(DataType::from("  bigint  ").unwrap(), DataType::BigInt);
        assert_eq!(DataType::from("\tchars\n").unwrap(), DataType::Chars);
        assert_eq!(
            DataType::from(" array/int ").unwrap(),
            DataType::Array("int".into())
        );
        // `to_arr` 是公开入口，直接调用时同样规范化
        assert_eq!(
            DataType::to_arr(" array/int ").unwrap(),
            DataType::Array("int".into())
        );
        assert_eq!(
            DataType::to_arr("  array  ").unwrap(),
            DataType::Array("auto".into())
        );
        // 全空白 → 与未知类型同等报错（不静默给 Auto）
        assert!(DataType::from("   ").is_err());
        // 大小写严格（未变）
        assert!(DataType::from("Int").is_err());
        assert!(DataType::from("ARRAY/int").is_err());
    }

    /// trim 的**边界**：把“刻意如此”的部分钉住，避免以后被“顺手全量 normalize”。
    #[test]
    fn test_trim_boundaries() {
        // `array` 与前缀 `/` 之间带空白
        assert_eq!(
            DataType::from("array /int").unwrap(),
            DataType::Array("int".into())
        );
        // Unicode 空白：`str::trim` 走 Unicode White_Space，NBSP / 表意空格 / 行分隔符都削
        for raw in [
            "\u{00A0}int\u{00A0}",
            "\u{3000}int\u{3000}",
            "\u{2028}int\u{2028}",
        ] {
            assert_eq!(DataType::from(raw).unwrap(), DataType::Int, "{raw:?}");
        }
        // …但 ZWSP（U+200B）不是 White_Space，不削 → 依旧是未知类型
        assert!(DataType::from("\u{200B}int").is_err());
        // 子类型只有制表/换行/空格 → auto
        assert_eq!(
            DataType::from("array/\t\n ").unwrap(),
            DataType::Array("auto".into())
        );
        // **内部空白刻意保留**（只削前后）
        assert_eq!(
            DataType::from("array/i nt").unwrap(),
            DataType::Array("i nt".into())
        );
        // 错误信息回显的是 trim 后的名字
        let err = DataType::from("  unknown  ").unwrap_err().to_string();
        assert!(err.contains("unknown meta: unknown"), "{err}");
        // 直接调用 `to_arr`（不经 `from`）的报错路径
        assert!(DataType::to_arr("").is_err());
        assert!(DataType::to_arr("   ").is_err());
        // 规范化是**类型不变量**：`ArraySubtype` 四条构造路径（new / From<&str> / From<String> /
        // serde 反序列化）都过规范化，所以带空格的 wire 输入与解析入口得到**同一个**类型。
        let wire: DataType = serde_json::from_str(r#"{"array":"int "}"#).unwrap();
        assert_eq!(wire, DataType::Array("int".into()));
        assert_eq!(wire, DataType::from("array/int ").unwrap());
        // 直接构造（不经解析）也逃不掉：`ArraySubtype` 载荷私有，`Array("int ".into())`
        // 进去就是 `Array("int")`。
        assert_eq!(
            DataType::Array("int ".into()),
            DataType::Array("int".into())
        );
        assert_eq!(ArraySubtype::new("  ").as_str(), "auto");
        assert_eq!(ArraySubtype::new("").as_str(), "auto");
        assert_eq!(ArraySubtype::new(" int ").as_str(), "int");
        // wire 写出同样是规范化的（不会再写出 `"array/int "`）
        assert_eq!(serde_json::to_string(&wire).unwrap(), r#"{"array":"int"}"#);
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
        assert_eq!(DataType::Int.static_name(), "int");
        assert_eq!(DataType::Float.static_name(), "float");
        assert_eq!(DataType::Time.static_name(), "time");
        assert_eq!(DataType::IP.static_name(), "ip");
        assert_eq!(DataType::Ignore.static_name(), "_");
        assert_eq!(DataType::Array("int".into()).static_name(), "array");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DataType::Bool), "bool");
        assert_eq!(format!("{}", DataType::Chars), "chars");
        assert_eq!(format!("{}", DataType::Array("int".into())), "array/int");
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
        assert!(DataType::Int.parse_patten_first());
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
            DataType::Int,
            DataType::BigInt,
            DataType::Float,
            DataType::Array("ip".into()),
        ];
        for dt in types {
            let json = serde_json::to_string(&dt).unwrap();
            let parsed: DataType = serde_json::from_str(&json).unwrap();
            assert_eq!(dt, parsed);
        }
    }

    /// 0.10.0 改名契约：`digit` → `int`。锁定解析名、显示名与 wire 名，以及「旧名被拒」。
    ///
    /// 这些断言是**故意**的破坏性契约：`DataType` 是裸 derive，serde 名即类型名，没有
    /// 兼容别名。若有人日后顺手把 `"digit"` 加回来当别名，本用例会失败。
    #[test]
    fn int_name_is_int_and_old_digit_is_rejected() {
        // 解析：新名可用，旧名退休
        assert_eq!(DataType::from("int").unwrap(), DataType::Int);
        assert!(DataType::from("digit").is_err(), "旧名 digit 已退休");
        // `int` 与 `bigint` 是两个类型，且不互为前缀误匹配（`from` 是精确 match）
        assert_eq!(DataType::from("bigint").unwrap(), DataType::BigInt);
        assert_ne!(
            DataType::from("int").unwrap(),
            DataType::from("bigint").unwrap()
        );
        // 静态名 / 显示 / wire 形态三者一致
        assert_eq!(DataType::Int.static_name(), "int");
        assert_eq!(format!("{}", DataType::Int), "int");
        assert_eq!(serde_json::to_string(&DataType::Int).unwrap(), r#""int""#);
        assert_eq!(
            serde_json::to_string(&DataType::BigInt).unwrap(),
            r#""bigint""#
        );
    }
}
