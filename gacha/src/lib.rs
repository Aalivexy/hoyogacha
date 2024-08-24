mod gacha_log;
mod gacha_url;
mod req;
pub use gacha_log::*;
pub use gacha_url::*;
pub use uigf;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub enum GameType {
    Hk4eCN,
    Hk4eGlobal,
    HkrpgCN,
    HkrpgGlobal,
    NapCN,
    NapGlobal,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GachaType {
    Hk4e(uigf::hk4e::GachaType),
    Hkrpg(uigf::hkrpg::GachaType),
    Nap(uigf::nap::GachaType),
}
