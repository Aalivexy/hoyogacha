use argh::FromArgs;

/// A command line tool for exporting gacha logs, supporting UIGF v4.0
#[derive(FromArgs, PartialEq, Debug)]
pub struct Args {
    /// specify the games to export
    #[argh(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Subcommand {
    Hk4e(SubcommandHk4e),
    Hkrpg(SubCommandHkrpg),
    Nap(SubCommandNap),
    Url(SubCommandUrl),
}

/// export gacha logs from Genshin Impact
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hk4e")]
pub struct SubcommandHk4e {
    /// manually specify the URL to export
    #[argh(option)]
    pub url: Option<String>,

    /// specify the file path to be exported
    /// (default: printed to the console)
    #[argh(positional)]
    pub output: Option<String>,

    /// specify the search to be global instead of cn
    /// (default: no)
    #[argh(switch, short = 'g')]
    pub global: bool,
}

/// export gacha logs from Honkai Star Rail
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "hkrpg")]
pub struct SubCommandHkrpg {
    /// manually specify the URL to export
    #[argh(option)]
    pub url: Option<String>,

    /// specify the file path to be exported
    /// (default: printed to the console)
    #[argh(positional)]
    pub output: Option<String>,

    /// specify the search to be global instead of cn
    /// (default: no)
    #[argh(switch, short = 'g')]
    pub global: bool,
}

/// export gacha logs from Zenless Zone Zero
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "nap")]
pub struct SubCommandNap {
    /// manually specify the URL to export
    #[argh(option)]
    pub url: Option<String>,

    /// specify the file path to be exported
    /// (default: printed to the console)
    #[argh(positional)]
    pub output: Option<String>,

    /// specify the search to be global instead of cn
    /// (default: no)
    #[argh(switch, short = 'g')]
    pub global: bool,
}

/// get url but not export UIGF
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "url")]
pub struct SubCommandUrl {
    /// specify game
    /// available options:
    /// - hk4ecn (原神)
    /// - hk4eglobal (Genshin Impact)
    /// - hkrpgcn (崩坏：星穹铁道)
    /// - hkrpgglobal (Honkai Star Rail)
    /// - napcn (绝区零)
    /// - napglobal (Zenless Zone Zero)

    #[argh(positional, from_str_fn(parse_game))]
    pub game: gacha::GameType,
}

pub fn parse_game(value: &str) -> Result<gacha::GameType, String> {
    match value {
        "hk4ecn" => Ok(gacha::GameType::Hk4eCN),
        "hk4eglobal" => Ok(gacha::GameType::Hk4eGlobal),
        "hkrpgcn" => Ok(gacha::GameType::HkrpgCN),
        "hkrpgglobal" => Ok(gacha::GameType::HkrpgGlobal),
        "napcn" => Ok(gacha::GameType::NapCN),
        "napglobal" => Ok(gacha::GameType::NapGlobal),
        _ => Err("invalid game".to_string()),
    }
}
