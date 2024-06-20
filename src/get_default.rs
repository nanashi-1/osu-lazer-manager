use anyhow::{anyhow, Ok};
use colored::Colorize;
use osu_lazer_manager::config::get_default_version;

pub fn get_default() -> anyhow::Result<()> {
    println!(
        "{}{}",
        "Default Version: ".green(),
        get_default_version()
            .map_err(|e| anyhow!("Failed to get default version: {}", e))?
            .as_str()
            .green()
    );

    Ok(())
}
