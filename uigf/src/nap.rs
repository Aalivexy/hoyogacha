use crate::{LanguageCode, Uid};
use serde::{Deserialize, Serialize};

/// 绝区零
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Nap {
    /// UID
    pub uid: Uid,
    /// 时区偏移，由米哈游 API 返回，若与服务器时区不同请注意 list 中 time 的转换
    pub timezone: i64,
    /// 语言代码
    pub lang: Option<LanguageCode>,
    /// 抽卡记录
    pub list: Vec<NapItem>,
}

/// 绝区零抽卡记录
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct NapItem {
    /// 卡池 Id
    pub gacha_id: Option<String>,
    /// 卡池类型
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
    /// 卡池类型
    GachaType {
        /// 常驻频段
        StableChannel => "1",
        /// 独家频段
        ExclusiveChannel => "2",
        /// 音擎频段
        WEngineChannel => "3",
        /// 邦布频段
        BangbooChannel => "5",
    }
}
