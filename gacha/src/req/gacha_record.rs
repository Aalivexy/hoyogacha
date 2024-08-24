use serde::{Deserialize, Serialize};
use uigf::{hk4e::Hk4eItem, hkrpg::HkrpgItem, LanguageCode, nap::NapItem};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct GachaRecord {
    pub uid: String,
    pub gacha_id: String,
    pub gacha_type: String,
    pub item_id: String,
    pub count: String,
    pub time: String,
    pub name: String,
    pub lang: LanguageCode,
    pub item_type: String,
    pub rank_type: String,
    pub id: String,
}

impl TryFrom<GachaRecord> for Hk4eItem {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: GachaRecord) -> Result<Self, Self::Error> {
        let gacha_type = value.gacha_type.parse()?;
        Ok(Self {
            gacha_type,
            uigf_gacha_type: gacha_type.to_uigf(),
            item_id: value.item_id,
            count: Some(value.count),
            time: value.time,
            name: Some(value.name),
            item_type: Some(value.item_type),
            rank_type: Some(value.rank_type),
            id: value.id,
        })
    }
}

impl TryFrom<GachaRecord> for HkrpgItem {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: GachaRecord) -> Result<Self, Self::Error> {
        let gacha_type = value.gacha_type.parse()?;
        Ok(Self {
            gacha_id: value.gacha_id,
            gacha_type,
            item_id: value.item_id,
            count: Some(value.count),
            time: value.time,
            name: Some(value.name),
            item_type: Some(value.item_type),
            rank_type: Some(value.rank_type),
            id: value.id,
        })
    }
}

impl TryFrom<GachaRecord> for NapItem {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: GachaRecord) -> Result<Self, Self::Error> {
        let gacha_type = value.gacha_type.parse()?;
        Ok(Self {
            gacha_id: Some(value.gacha_id),
            gacha_type,
            item_id: value.item_id,
            count: Some(value.count),
            time: value.time,
            name: Some(value.name),
            item_type: Some(value.item_type),
            rank_type: Some(value.rank_type),
            id: Some(value.id),
        })
    }
}
