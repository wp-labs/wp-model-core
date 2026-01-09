use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum OutFmt {
    #[serde(rename = "fmt")]
    Fmt(TextFmt),
}
impl Default for OutFmt {
    fn default() -> Self {
        OutFmt::Fmt(TextFmt::default())
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default, Copy)]
pub enum TextFmt {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "show")]
    Show,
    #[serde(rename = "kv")]
    Kv,
    #[serde(rename = "raw")]
    #[default]
    Raw,
    #[serde(rename = "proto")]
    Proto,
    #[serde(rename = "proto-text")]
    ProtoText,
}

impl From<&str> for TextFmt {
    fn from(value: &str) -> Self {
        if value == "json" {
            TextFmt::Json
        } else if value == "csv" {
            TextFmt::Csv
        } else if value == "show" {
            TextFmt::Show
        } else if value == "kv" {
            TextFmt::Kv
        } else if value == "proto" {
            TextFmt::Proto
        } else if value == "proto-text" {
            TextFmt::ProtoText
        } else {
            TextFmt::Raw
        }
    }
}

impl Display for TextFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextFmt::Json => write!(f, "json"),
            TextFmt::Csv => write!(f, "csv"),
            TextFmt::Show => write!(f, "show"),
            TextFmt::Kv => write!(f, "kv"),
            TextFmt::Raw => write!(f, "raw"),
            TextFmt::Proto => write!(f, "proto"),
            TextFmt::ProtoText => write!(f, "proto-text"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== TextFmt tests ==========

    #[test]
    fn test_text_fmt_default() {
        assert_eq!(TextFmt::default(), TextFmt::Raw);
    }

    #[test]
    fn test_text_fmt_from_str() {
        assert_eq!(TextFmt::from("json"), TextFmt::Json);
        assert_eq!(TextFmt::from("csv"), TextFmt::Csv);
        assert_eq!(TextFmt::from("show"), TextFmt::Show);
        assert_eq!(TextFmt::from("kv"), TextFmt::Kv);
        assert_eq!(TextFmt::from("proto"), TextFmt::Proto);
        assert_eq!(TextFmt::from("proto-text"), TextFmt::ProtoText);
    }

    #[test]
    fn test_text_fmt_from_str_unknown() {
        // Unknown values should default to Raw
        assert_eq!(TextFmt::from("unknown"), TextFmt::Raw);
        assert_eq!(TextFmt::from(""), TextFmt::Raw);
        assert_eq!(TextFmt::from("JSON"), TextFmt::Raw); // case sensitive
    }

    #[test]
    fn test_text_fmt_display() {
        assert_eq!(format!("{}", TextFmt::Json), "json");
        assert_eq!(format!("{}", TextFmt::Csv), "csv");
        assert_eq!(format!("{}", TextFmt::Show), "show");
        assert_eq!(format!("{}", TextFmt::Kv), "kv");
        assert_eq!(format!("{}", TextFmt::Raw), "raw");
        assert_eq!(format!("{}", TextFmt::Proto), "proto");
        assert_eq!(format!("{}", TextFmt::ProtoText), "proto-text");
    }

    #[test]
    fn test_text_fmt_roundtrip() {
        // from -> display should be consistent
        let formats = ["json", "csv", "show", "kv", "proto", "proto-text"];
        for fmt_str in formats {
            let fmt = TextFmt::from(fmt_str);
            assert_eq!(format!("{}", fmt), fmt_str);
        }
    }

    // ========== OutFmt tests ==========

    #[test]
    fn test_out_fmt_default() {
        let out = OutFmt::default();
        assert_eq!(out, OutFmt::Fmt(TextFmt::Raw));
    }

    #[test]
    fn test_out_fmt_with_text_fmt() {
        let out = OutFmt::Fmt(TextFmt::Json);
        assert_eq!(out, OutFmt::Fmt(TextFmt::Json));
    }

    // ========== Serde tests ==========

    #[test]
    fn test_text_fmt_serde() {
        let fmt = TextFmt::Json;
        let json = serde_json::to_string(&fmt).unwrap();
        assert_eq!(json, "\"json\"");

        let parsed: TextFmt = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, TextFmt::Json);
    }

    #[test]
    fn test_text_fmt_serde_all_variants() {
        let variants = vec![
            (TextFmt::Json, "\"json\""),
            (TextFmt::Csv, "\"csv\""),
            (TextFmt::Show, "\"show\""),
            (TextFmt::Kv, "\"kv\""),
            (TextFmt::Raw, "\"raw\""),
            (TextFmt::Proto, "\"proto\""),
            (TextFmt::ProtoText, "\"proto-text\""),
        ];

        for (fmt, expected_json) in variants {
            let json = serde_json::to_string(&fmt).unwrap();
            assert_eq!(json, expected_json);

            let parsed: TextFmt = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, fmt);
        }
    }

    #[test]
    fn test_out_fmt_serde() {
        let out = OutFmt::Fmt(TextFmt::Csv);
        let json = serde_json::to_string(&out).unwrap();
        let parsed: OutFmt = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, out);
    }

    // ========== Clone and PartialEq tests ==========

    #[test]
    fn test_text_fmt_clone() {
        let fmt = TextFmt::Json;
        let cloned = fmt;
        assert_eq!(fmt, cloned);
    }

    #[test]
    fn test_out_fmt_clone() {
        let out = OutFmt::Fmt(TextFmt::Kv);
        let cloned = out.clone();
        assert_eq!(out, cloned);
    }
}
