use gacha::get_gacha_url;

mod args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg: args::Args = argh::from_env();
    match arg.subcommand {
        args::Subcommand::Hk4e(cmd) => {
            process_command(
                cmd.url,
                cmd.output,
                cmd.global,
                gacha::GameType::Hk4eGlobal,
                gacha::GameType::Hk4eCN,
            )?;
        }
        args::Subcommand::Hkrpg(cmd) => {
            process_command(
                cmd.url,
                cmd.output,
                cmd.global,
                gacha::GameType::HkrpgGlobal,
                gacha::GameType::HkrpgCN,
            )?;
        }
        args::Subcommand::Nap(cmd) => {
            process_command(
                cmd.url,
                cmd.output,
                cmd.global,
                gacha::GameType::NapGlobal,
                gacha::GameType::NapCN,
            )?;
        }
        args::Subcommand::Url(cmd) => {
            println!("{}", get_gacha_url(cmd.game)?);
        }
    }
    Ok(())
}

fn process_command(
    url_option: Option<String>,
    output: Option<String>,
    global: bool,
    global_game_type: gacha::GameType,
    cn_game_type: gacha::GameType,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = if let Some(s) = url_option {
        url::Url::parse(&s)?
    } else {
        gacha::get_gacha_url(if global {
            global_game_type
        } else {
            cn_game_type
        })?
    };
    let uigf = gacha::get_uigf_with_url_all(
        if global {
            global_game_type
        } else {
            cn_game_type
        },
        url,
    )?
    .to_json()?;
    match output {
        Some(p) => std::fs::write(p, uigf)?,
        None => println!("{}", uigf),
    }
    Ok(())
}
