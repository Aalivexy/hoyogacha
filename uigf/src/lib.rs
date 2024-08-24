#[macro_use]
mod enum_with_str;
mod utils;

pub mod hk4e;
pub mod hkrpg;
pub mod nap;
pub use utils::*;
pub use enum_with_str::AllVariants;

/// 统一可交换抽卡记录标准 v4.0
/// https://uigf.org/standards/uigf.html
#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UigfV4 {
    pub info: Info,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hk4e: Option<Vec<hk4e::Hk4e>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkrpg: Option<Vec<hkrpg::Hkrpg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nap: Option<Vec<nap::Nap>>,
}

/// 导出档案的 App 名称
#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Info {
    /// 导出档案的时间戳，秒级
    pub export_timestamp: ExportTimestamp,
    /// 导出档案的 App 名称
    pub export_app: String,
    /// 导出档案的 App 版本
    pub export_app_version: String,
    /// 导出档案的 UIGF 版本号，格式为 'v{major}.{minor}'，如 v4.0
    pub version: String,
}

impl UigfV4 {
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json)?)
    }
}

impl Default for Info {
    fn default() -> Self {
        Self {
            export_timestamp: ExportTimestamp::now(),
            export_app: env!("CARGO_CRATE_NAME").into(),
            export_app_version: env!("CARGO_PKG_VERSION").into(),
            version: "v4.0".into(),
        }
    }
}
