use crate::{hk4e::Hk4e, hkrpg::Hkrpg, nap::Nap, Info, UigfV4};
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

impl UigfV4 {
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn new() -> Self {
        Self {
            info: Info::new(),
            hk4e: None,
            hkrpg: None,
            nap: None,
        }
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
