use anyhow::Ok;
use colored::Colorize;
use osu_lazer_manager::config::get_default_version;

pub fn get_default() -> anyhow::Result<()> {
    println!(
        "{}{}",
        "Default Version: ".green(),
        get_default_version()?.as_str().green()
    );

    Ok(())
}
