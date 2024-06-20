use anyhow::Ok;
use colored::Colorize;
use osu_lazer_manager::{
    config::set_default_version,
    fetcher::{
        appimage::get_appimage_path,
        desktop::{get_desktop_cache_path, get_desktop_path, process_template},
        icon::get_icon_path,
    },
    utils::version::get_latest_version,
};
use std::fs::{read_to_string, write};

pub async fn set_default(version: &str) -> anyhow::Result<()> {
    let version = match version {
        "latest" => get_latest_version().await?,
        _ => version.to_string(),
    };

    set_default_version(&version)?;
    replace_desktop_version(&version)?;

    println!("{}{}", "Set default version: ".green(), version.green());

    Ok(())
}

fn replace_desktop_version(version: &str) -> anyhow::Result<()> {
    let desktop_cache_path = get_desktop_cache_path()?;
    let desktop_template = read_to_string(desktop_cache_path)?;

    let output_desktop = process_template(
        &desktop_template,
        get_icon_path()?.to_str().unwrap(),
        get_appimage_path(&version)?.to_str().unwrap(),
    );

    write(get_desktop_path()?, output_desktop)?;

    Ok(())
}
