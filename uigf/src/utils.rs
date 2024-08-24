use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

enum_with_str! {
    /// 语言代码
    LanguageCode {
        DeDe => "de-de",
        EnUs => "en-us",
        EsEs => "es-es",
        FrFr => "fr-fr",
        IdId => "id-id",
        ItIt => "it-it",
        JaJp => "ja-jp",
        KoKr => "ko-kr",
        PtPt => "pt-pt",
        RuRu => "ru-ru",
        ThTh => "th-th",
        TrTr => "tr-tr",
        ViVn => "vi-vn",
        ZhCn => "zh-cn",
        ZhTw => "zh-tw"
    }
}

/// 导出档案的时间戳，秒级
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExportTimestamp {
    String(String),
    Integer(u64),
}

/// UID
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Uid {
    String(String),
    Integer(u64),
}

impl ExportTimestamp {
    pub fn now() -> Self {
        ExportTimestamp::Integer(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}

impl Display for ExportTimestamp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ExportTimestamp::String(s) => write!(f, "{}", s),
            ExportTimestamp::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl std::str::FromStr for ExportTimestamp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<u64>() {
            return Ok(ExportTimestamp::Integer(i));
        }
        Ok(ExportTimestamp::String(s.to_string()))
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Uid::String(s) => write!(f, "{}", s),
            Uid::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl std::str::FromStr for Uid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<u64>() {
            return Ok(Uid::Integer(i));
        }
        Ok(Uid::String(s.to_string()))
    }
}
