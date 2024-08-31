use crate::{
    get_gacha_url,
    req::{fetch_gacha_log, filter_url, ResponseData},
    GachaType, GameType,
};
use std::error::Error;
use uigf::{hk4e::Hk4e, hkrpg::Hkrpg, nap::Nap, Info, UigfV4};
use url::Url;

pub fn get_uigf_all(game_type: GameType) -> Result<UigfV4, Box<dyn Error>> {
    get_uigf_with_url_all(game_type, get_gacha_url(game_type)?)
}

pub fn get_uigf_with_url_all(game_type: GameType, url: Url) -> Result<UigfV4, Box<dyn Error>> {
    match game_type {
        GameType::Hk4eCN | GameType::Hk4eGlobal => {
            let uigf_data = uigf::hk4e::GachaType::all_variants()
                .iter()
                .map(|gacha_type| get_uigf_with_url(GachaType::Hk4e(*gacha_type), url.clone()))
                .filter_map(|uigf_data| uigf_data.ok())
                .collect::<Vec<_>>();
            let data = uigf_data
                .iter()
                .filter_map(|uigf_data| uigf_data.hk4e.clone())
                .flatten()
                .collect::<Vec<_>>();
            if data.is_empty() {
                Err("No data found".into())
            } else {
                Ok(UigfV4::new_hk4e(vec![Hk4e {
                    uid: data[0].uid.clone(),
                    timezone: data[0].timezone,
                    lang: data[0].lang,
                    list: data.into_iter().map(|data| data.list).flatten().collect(),
                }]))
            }
        }
        GameType::HkrpgCN | GameType::HkrpgGlobal => {
            let uigf_data = uigf::hkrpg::GachaType::all_variants()
                .iter()
                .map(|gacha_type| get_uigf_with_url(GachaType::Hkrpg(*gacha_type), url.clone()))
                .filter_map(|uigf_data| uigf_data.ok())
                .collect::<Vec<_>>();
            let data = uigf_data
                .iter()
                .filter_map(|uigf_data| uigf_data.hkrpg.clone())
                .flatten()
                .collect::<Vec<_>>();
            if data.is_empty() {
                Err("No data found".into())
            } else {
                Ok(UigfV4::new_hkrpg(vec![Hkrpg {
                    uid: data[0].uid.clone(),
                    timezone: data[0].timezone,
                    lang: data[0].lang,
                    list: data.into_iter().map(|data| data.list).flatten().collect(),
                }]))
            }
        }
        GameType::NapCN | GameType::NapGlobal => {
            let uigf_data = uigf::nap::GachaType::all_variants()
                .iter()
                .map(|gacha_type| get_uigf_with_url(GachaType::Nap(*gacha_type), url.clone()))
                .filter_map(|uigf_data| uigf_data.ok())
                .collect::<Vec<_>>();
            let data = uigf_data
                .iter()
                .filter_map(|uigf_data| uigf_data.nap.clone())
                .flatten()
                .collect::<Vec<_>>();
            if data.is_empty() {
                Err("No data found".into())
            } else {
                Ok(UigfV4::new_nap(vec![Nap {
                    uid: data[0].uid.clone(),
                    timezone: data[0].timezone,
                    lang: data[0].lang,
                    list: data.into_iter().map(|data| data.list).flatten().collect(),
                }]))
            }
        }
    }
}

pub fn get_uigf(game_type: GameType, gacha_type: GachaType) -> Result<UigfV4, Box<dyn Error>> {
    get_uigf_with_gacha_log(gacha_type, vec![get_gacha_log(game_type, gacha_type)?])
}

pub fn get_uigf_with_url(gacha_type: GachaType, url: Url) -> Result<UigfV4, Box<dyn Error>> {
    get_uigf_with_gacha_log(gacha_type, vec![get_gacha_log_with_url(gacha_type, url)?])
}

pub fn get_uigf_with_gacha_log(
    gacha_type: GachaType,
    gacha_log: Vec<ResponseData>,
) -> Result<UigfV4, Box<dyn Error>> {
    fn collect_gacha_log<T: TryFrom<ResponseData, Error = Box<dyn Error>>>(
        gacha_log: Vec<ResponseData>,
    ) -> Option<Vec<T>> {
        Some(
            gacha_log
                .into_iter()
                .filter_map(|resp: ResponseData| T::try_from(resp).ok())
                .collect::<Vec<_>>(),
        )
    }

    let (hk4e, hkrpg, nap) = match gacha_type {
        GachaType::Hk4e(_) => (collect_gacha_log::<Hk4e>(gacha_log), None, None),
        GachaType::Hkrpg(_) => (None, collect_gacha_log::<Hkrpg>(gacha_log), None),
        GachaType::Nap(_) => (None, None, collect_gacha_log::<Nap>(gacha_log)),
    };

    Ok(UigfV4 {
        info: Info::new(),
        hk4e,
        hkrpg,
        nap,
    })
}

pub fn get_gacha_log(
    game_type: GameType,
    gacha_type: GachaType,
) -> Result<ResponseData, Box<dyn Error>> {
    get_gacha_log_with_url(gacha_type, get_gacha_url(game_type)?)
}

pub fn get_gacha_log_with_url(
    gacha_type: GachaType,
    url: Url,
) -> Result<ResponseData, Box<dyn Error>> {
    let mut url = filter_url(url);
    let gacha_type = match gacha_type {
        GachaType::Hk4e(t) => t.to_string(),
        GachaType::Hkrpg(t) => t.to_string(),
        GachaType::Nap(t) => t.to_string(),
    };
    url.query_pairs_mut()
        .append_pair("gacha_type", &gacha_type)
        .append_pair("real_gacha_type", &gacha_type)
        .append_pair("size", "20");
    Ok(fetch_gacha_log(url).unwrap())
}
