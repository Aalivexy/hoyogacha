use crate::{req::check_url, GameType};
use regex_lite::Regex;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};
use url::Url;

pub fn get_gacha_url(game_type: GameType) -> Result<Url, Box<dyn Error>> {
    let log_path = get_local_app_data_low_folder()?.join(match game_type {
        GameType::Hk4eCN => "miHoYo/原神/output_log.txt",
        GameType::Hk4eGlobal => "miHoYo/Genshin Impact/output_log.txt",
        GameType::HkrpgCN => "miHoYo/崩坏：星穹铁道/Player.log",
        GameType::HkrpgGlobal => "miHoYo/Cognosphere/Star Rail/Player.log",
        GameType::NapCN => "miHoYo/绝区零/Player.log",
        GameType::NapGlobal => "miHoYo/ZenlessZoneZero/Player.log",
    });

    get_gacha_url_with_log_data(fs::read_to_string(log_path)?)
}

pub fn get_gacha_url_with_log_data(log_data: String) -> Result<Url, Box<dyn Error>> {
    let re = Regex::new(
        r"([A-Z]:/.*?(GenshinImpact_Data|YuanShen_Data|StarRail_Data|ZenlessZoneZero_Data))",
    )?;
    let captures = re.captures(&log_data).ok_or("Game data path not found")?;
    get_gacha_url_with_game_data_path(&captures[0])
}

pub fn get_gacha_url_with_game_data_path(
    game_data_path: impl AsRef<Path>,
) -> Result<Url, Box<dyn Error>> {
    let re = Regex::new(r"(https://.+?/api/getGachaLog.+?authkey=.+?end_id=)")?;

    // data_2 contains many non-UTF-8 characters, we only need the UTF-8 part
    String::from_utf8_lossy(&fs::read(
        get_latest_folder(game_data_path.as_ref().join("webCaches"))?
            .join("Cache/Cache_Data/data_2"),
    )?)
    .to_string()
    .split("1/0/")
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
    .filter_map(|line| re.captures(line))
    .filter(|cap| cap.len() >= 1)
    .filter_map(|cap| Url::from_str(&cap[0]).ok())
    .find_map(check_url)
    .ok_or("No valid URL found".into())
}

fn get_local_app_data_low_folder() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(
        windows::Storage::UserDataPaths::GetDefault()?
            .LocalAppDataLow()?
            .to_os_string(),
    ))
}

fn get_latest_folder(path: impl AsRef<Path>) -> Result<PathBuf, Box<dyn Error>> {
    fs::read_dir(path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .max_by_key(|entry| entry.metadata().and_then(|meta| meta.modified()).ok())
        .map(|entry| entry.path())
        .ok_or_else(|| "No directories found".into())
}
