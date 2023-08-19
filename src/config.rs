use ini::configparser::ini::Ini;

use crate::error::Result;
use std::{fs::metadata, path::PathBuf};

const SECTION: &str = "main";

#[derive(Clone)]
pub struct Config {
    pub version: String,
    pub config_path: PathBuf,
}

impl Config {
    pub fn save(&self) -> Result<()> {
        let mut ini = Ini::new();

        ini.set(SECTION, "version", Some(self.version.to_owned()));

        ini.write(
            self.config_path
                .to_str()
                .ok_or("Error converting path to string slice.")?,
        )?;

        Ok(())
    }

    /// *Note: Will create a config file if config doesn't exist.*
    pub fn load(config_path: PathBuf) -> Result<Self> {
        if metadata(&config_path).is_err() {
            let config = Self {
                config_path,
                ..Default::default()
            };
            Self::save(&config)?;
            return Ok(config);
        }

        let mut ini = Ini::new();
        ini.load(
            config_path
                .to_str()
                .ok_or("Error converting path to string slice.")?,
        )?;

        Ok(Self {
            version: ini.get(SECTION, "version").ok_or("Not Installed")?,
            config_path,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "Not Installed".into(),
            config_path: Default::default(),
        }
    }
}
