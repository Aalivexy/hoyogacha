use crate::{hk4e::Hk4e, hkrpg::Hkrpg, nap::Nap, Info, UigfV4};
use serde::{Deserialize, Serialize};
use std::{
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
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

impl UigfV4 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_hk4e(hk4e: Vec<Hk4e>) -> Self {
        Self {
            info: Info::new(),
            hk4e: Some(hk4e),
            hkrpg: None,
            nap: None,
        }
    }

    pub fn new_hkrpg(hkrpg: Vec<Hkrpg>) -> Self {
        Self {
            info: Info::new(),
            hk4e: None,
            hkrpg: Some(hkrpg),
            nap: None,
        }
    }

    pub fn new_nap(nap: Vec<Nap>) -> Self {
        Self {
            info: Info::new(),
            hk4e: None,
            hkrpg: None,
            nap: Some(nap),
        }
    }
}

impl Info {
    pub fn new() -> Self {
        Self {
            export_timestamp: ExportTimestamp::now(),
            export_app: concat!("lib", env!("CARGO_CRATE_NAME")).into(),
            export_app_version: env!("CARGO_PKG_VERSION").into(),
            version: "v4.0".into(),
        }
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportTimestamp {
    pub fn now() -> Self {
        Self::Integer(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        )
    }
}

impl Default for ExportTimestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl Display for ExportTimestamp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl FromStr for ExportTimestamp {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u64>()
            .map(Self::Integer)
            .unwrap_or_else(|_| Self::String(s.to_string())))
    }
}

impl Default for Uid {
    fn default() -> Self {
        Self::Integer(0)
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Integer(i) => write!(f, "{}", i),
        }
    }
}

impl FromStr for Uid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<u64>()
            .map(Self::Integer)
            .unwrap_or_else(|_| Self::String(s.to_string())))
    }
}
