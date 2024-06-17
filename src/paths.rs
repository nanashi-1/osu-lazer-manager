use anyhow::{anyhow, Ok};

use crate::constants::{APPLICATION_DIRECTORY, DIRECTORY_NAME};
use std::path::PathBuf;

pub fn get_application_path() -> anyhow::Result<PathBuf> {
    Ok(get_data_dir()?.join(APPLICATION_DIRECTORY))
}

pub fn get_directory_path() -> anyhow::Result<PathBuf> {
    Ok(get_data_dir()?.join(DIRECTORY_NAME))
}

fn get_data_dir() -> anyhow::Result<PathBuf> {
    dirs::data_dir().ok_or(anyhow!("Cannot get data directory."))
}
