use super::GachaRecord;
use serde::{Deserialize, Serialize};
use uigf::{
    hk4e::{Hk4e, Hk4eItem},
    hkrpg::{Hkrpg, HkrpgItem},
    nap::{Nap, NapItem},
};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Response {
    pub retcode: i64,
    pub message: String,
    pub data: Option<ResponseData>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct ResponseData {
    pub page: Option<String>,
    pub size: String,
    pub list: Vec<GachaRecord>,
    pub region: Option<String>,
    pub region_time_zone: Option<i64>,
}

fn get_time_zone_by_uid(uid: &str) -> i64 {
    // 原神不提供时区信息，根据 UID 判断
    // 6 => os_usa, 7 => os_euro, _ => os_cht, os_asia, cn_gf01, cn_qd01
    if uid.starts_with("6") {
        -5
    } else if uid.starts_with("7") {
        1
    } else {
        8
    }
}

impl TryFrom<ResponseData> for Hk4e {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: ResponseData) -> Result<Self, Self::Error> {
        if value.list.is_empty() {
            return Err("No data found".into());
        }
        Ok(Self {
            uid: value.list[0].uid.parse()?,
            timezone: value
                .region_time_zone
                .unwrap_or(get_time_zone_by_uid(&value.list[0].uid)),
            lang: Some(value.list[0].lang.clone()),
            list: value
                .list
                .into_iter()
                .map(Hk4eItem::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ResponseData> for Hkrpg {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: ResponseData) -> Result<Self, Self::Error> {
        if value.list.is_empty() {
            return Err("No data found".into());
        }
        dbg!(value.clone());
        Ok(Self {
            uid: value.list[0].uid.parse()?,
            timezone: value.region_time_zone.unwrap(),
            lang: Some(value.list[0].lang.clone()),
            list: value
                .list
                .into_iter()
                .map(HkrpgItem::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<ResponseData> for Nap {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: ResponseData) -> Result<Self, Self::Error> {
        if value.list.is_empty() {
            return Err("No data found".into());
        }
        Ok(Self {
            uid: value.list[0].uid.parse()?,
            timezone: value.region_time_zone.unwrap(),
            lang: Some(value.list[0].lang.clone()),
            list: value
                .list
                .into_iter()
                .map(NapItem::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
