use anyhow::Ok;
use colored::Colorize;
use osu_lazer_manager::{config::set_default_version, utils::version::get_latest_version};

pub async fn set_default(version: &str) -> anyhow::Result<()> {
    let version = match version {
        "latest" => get_latest_version().await?,
        _ => version.to_string(),
    };

    set_default_version(&version)?;

    println!("{}{}", "Set default version: ".green(), version.green());

    Ok(())
}
