use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum RawData {
    String(String),
    Bytes(Bytes),
    ArcBytes(Arc<Vec<u8>>),
}

impl RawData {
    pub fn from_string<T: Into<String>>(value: T) -> RawData {
        RawData::String(value.into())
    }

    pub fn from_arc_bytes(data: Arc<Vec<u8>>) -> Self {
        RawData::ArcBytes(data)
    }

    /// 辅助构造：从 `Arc<[u8]>` 构建。该接口用于兼容旧版（0.4.6 之前）`ArcBytes` 表示，
    /// 会额外复制一次数据，建议尽快迁移到 `Arc<Vec<u8>>`。
    pub fn from_arc_slice(data: Arc<[u8]>) -> Self {
        RawData::ArcBytes(Arc::new(data.as_ref().to_vec()))
    }

    // 统一的数据访问接口
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            RawData::String(s) => s.as_bytes(),
            RawData::Bytes(b) => b.as_ref(),
            RawData::ArcBytes(arc) => arc.as_slice(),
        }
    }

    // 向后兼容的 Bytes 转换（仅在需要时，始终复制）
    pub fn to_bytes(&self) -> Bytes {
        match self {
            RawData::String(s) => Bytes::copy_from_slice(s.as_bytes()),
            RawData::Bytes(b) => b.clone(),
            RawData::ArcBytes(arc) => Bytes::copy_from_slice(arc.as_slice()),
        }
    }

    /// 按需取得 Bytes，消耗自身以在 `String`/`Bytes` 分支复用缓冲区。
    pub fn into_bytes(self) -> Bytes {
        match self {
            RawData::String(s) => Bytes::from(s),
            RawData::Bytes(b) => b,
            RawData::ArcBytes(arc) => match Arc::try_unwrap(arc) {
                Ok(vec) => Bytes::from(vec),
                Err(shared) => Bytes::copy_from_slice(shared.as_slice()),
            },
        }
    }

    // 零拷贝检测
    pub fn is_zero_copy(&self) -> bool {
        matches!(self, RawData::ArcBytes(_))
    }

    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        match self {
            RawData::String(value) => value.is_empty(),
            RawData::Bytes(value) => value.is_empty(),
            RawData::ArcBytes(arc) => arc.is_empty(),
        }
    }
}

impl Display for RawData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RawData::String(value) => f.write_str(value),
            // 安全转换：尽量显示为 UTF-8；不可解码时使用替代字符
            RawData::Bytes(value) => f.write_str(&String::from_utf8_lossy(value)),
            RawData::ArcBytes(arc) => f.write_str(&String::from_utf8_lossy(arc.as_slice())),
        }
    }
}
