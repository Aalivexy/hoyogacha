use serde::{Deserialize, Serialize};
use uigf::{hk4e::Hk4eItem, hkrpg::HkrpgItem, nap::NapItem, EnumParseError, LanguageCode};

#[derive(thiserror::Error, Debug)]
pub enum GachaRecordError {
    #[error("Invalid gacha type: {0}")]
    InvalidGachaType(String),
    #[error("Missing required field: gacha_id")]
    MissingGachaId,
    #[error("Enum parse error: {0}")]
    EnumParseError(#[from] EnumParseError),
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct GachaRecord {
    pub uid: String,
    pub gacha_id: Option<String>,
    pub gacha_type: String,
    pub item_id: String,
    pub count: Option<String>,
    pub time: String,
    pub name: Option<String>,
    pub lang: LanguageCode,
    pub item_type: Option<String>,
    pub rank_type: Option<String>,
    pub id: String,
}

impl GachaRecord {
    fn parse_gacha_type<T: std::str::FromStr<Err = EnumParseError>>(
        &self,
    ) -> Result<T, GachaRecordError> {
        self.gacha_type
            .parse()
            .map_err(GachaRecordError::EnumParseError)
    }
}

impl TryFrom<GachaRecord> for Hk4eItem {
    type Error = GachaRecordError;

    fn try_from(value: GachaRecord) -> Result<Self, Self::Error> {
        let gacha_type = value.parse_gacha_type()?;

        Ok(Self {
            gacha_type,
            uigf_gacha_type: gacha_type.into(),
            item_id: value.item_id,
            count: value.count,
            time: value.time,
            name: value.name,
            item_type: value.item_type,
            rank_type: value.rank_type,
            id: value.id,
        })
    }
}

impl TryFrom<GachaRecord> for HkrpgItem {
    type Error = GachaRecordError;

    fn try_from(value: GachaRecord) -> Result<Self, Self::Error> {
        let gacha_type = value.parse_gacha_type()?;
        let gacha_id = value.gacha_id.ok_or(GachaRecordError::MissingGachaId)?;

        Ok(Self {
            gacha_id,
            gacha_type,
            item_id: value.item_id,
            count: value.count,
            time: value.time,
            name: value.name,
            item_type: value.item_type,
            rank_type: value.rank_type,
            id: value.id,
        })
    }
}

impl TryFrom<GachaRecord> for NapItem {
    type Error = GachaRecordError;

    fn try_from(value: GachaRecord) -> Result<Self, Self::Error> {
        let gacha_type = value.parse_gacha_type()?;

        Ok(Self {
            gacha_id: value.gacha_id,
            gacha_type,
            item_id: value.item_id,
            count: value.count,
            time: value.time,
            name: value.name,
            item_type: value.item_type,
            rank_type: value.rank_type,
            id: value.id,
        })
    }
}
