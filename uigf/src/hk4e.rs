use crate::{LanguageCode, Uid};
use serde::{Deserialize, Serialize};

/// HK4E 原神
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Hk4e {
    /// UID
    pub uid: Uid,
    /// 时区偏移，由米哈游 API 返回，若与服务器时区不同请注意 list 中 time 的转换
    pub timezone: i64,
    /// 语言代码
    pub lang: Option<LanguageCode>,
    /// 抽卡记录
    pub list: Vec<Hk4eItem>,
}

/// 原神抽卡记录
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Hk4eItem {
    /// UIGF 卡池类型，用于区分卡池类型不同，但卡池保底计算相同的物品
    pub uigf_gacha_type: UigfGachaType,
    /// 卡池类型，米哈游 API 返回
    pub gacha_type: GachaType,
    /// 物品的内部 ID
    pub item_id: String,
    /// 物品个数，一般为1，米哈游 API 返回
    pub count: Option<String>,
    /// 抽取物品时对应时区（timezone）下的当地时间
    pub time: String,
    /// 物品名称，米哈游 API 返回
    pub name: Option<String>,
    /// 物品类型，米哈游 API 返回
    pub item_type: Option<String>,
    /// 物品等级，米哈游 API 返回
    pub rank_type: Option<String>,
    /// 记录内部 ID，米哈游 API 返回
    pub id: String,
}

enum_with_str! {
    /// UIGF 卡池类型，用于区分卡池类型不同，但卡池保底计算相同的物品
    UigfGachaType {
        /// 常驻祈愿
        PermanentWish => "100",
        /// 新手祈愿
        NoviceWishes => "200",
        /// 角色活动祈愿
        CharacterEventWish => "301",
        /// 武器活动祈愿
        WeaponEventWish => "302",
        /// 集录祈愿
        ChronicledWish => "500",
    }
}

enum_with_str! {
    /// 卡池类型
    GachaType {
        /// 常驻祈愿
        PermanentWish => "100",
        /// 新手祈愿
        NoviceWishes => "200",
        /// 角色活动祈愿
        CharacterEventWish => "301",
        /// 武器活动祈愿
        WeaponEventWish => "302",
        /// 角色活动祈愿 2
        CharacterEventWish2 => "400",
        /// 集录祈愿
        ChronicledWish => "500",
    }
}

impl GachaType {
    pub fn to_uigf(&self) -> UigfGachaType {
        match self {
            GachaType::PermanentWish => UigfGachaType::PermanentWish,
            GachaType::NoviceWishes => UigfGachaType::NoviceWishes,
            GachaType::CharacterEventWish => UigfGachaType::CharacterEventWish,
            GachaType::WeaponEventWish => UigfGachaType::WeaponEventWish,
            GachaType::CharacterEventWish2 => UigfGachaType::CharacterEventWish,
            GachaType::ChronicledWish => UigfGachaType::ChronicledWish,
        }
    }
}
