use crate::{constants::CONFIG, paths::get_directory_path};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{fs::write, path::PathBuf};

#[derive(Serialize, Deserialize, Default)]
struct Config {
    default_version: Option<String>,
}

fn get_config_path() -> anyhow::Result<PathBuf> {
    let config_path = get_directory_path()?.join(CONFIG);

    if !config_path.exists() {
        write(&config_path, toml::to_string(&Config::default())?)?;
    }

    Ok(config_path)
}

pub fn set_default_version(specific_version: &str) -> anyhow::Result<()> {
    let config_path = get_config_path()?;

    let mut config: Config = toml::from_str(&std::fs::read_to_string(&config_path)?)?;
    config.default_version = Some(specific_version.to_string());

    write(config_path, toml::to_string(&config)?)?;

    Ok(())
}

pub fn get_default_version() -> anyhow::Result<String> {
    let config_path = get_config_path()?;
    let config: Config = toml::from_str(&std::fs::read_to_string(config_path)?)?;

    match config.default_version {
        Some(version) => Ok(version),
        None => Err(anyhow!("No default version set")),
    }
}
