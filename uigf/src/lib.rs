#[macro_use]
mod enum_with_str;
mod utils;

pub mod hk4e;
pub mod hkrpg;
pub mod nap;
pub use utils::*;
pub use enum_with_str::AllVariants;

/// 统一可交换抽卡记录标准 v4.0
/// h<ttps://uigf.org/standards/uigf.html>
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct UigfV4 {
    pub info: Info,
    pub hk4e: Option<Vec<hk4e::Hk4e>>,
    pub hkrpg: Option<Vec<hkrpg::Hkrpg>>,
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
