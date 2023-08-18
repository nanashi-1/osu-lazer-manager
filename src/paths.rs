use crate::error::Result;
use std::path::PathBuf;

/// Path of files and directories installed by this installer.
pub enum Paths {
    Config,
    AppImage,
    DesktopFile,
    DesktopIcon,
    Manager,
}

/// Get the path of the requested file or directory.
pub fn get_path(target_file: Paths) -> Result<PathBuf> {
    let error_message = "Failed to get directory.";

    match target_file {
        Paths::Config => Ok(dirs::config_dir()
            .ok_or(error_message)?
            .join("osu-lazer-manager.cfg")),
        Paths::AppImage => Ok(dirs::data_dir()
            .ok_or(error_message)?
            .join("osu-lazer-manager/osu.AppImage")),
        Paths::DesktopFile => Ok(dirs::data_dir()
            .ok_or(error_message)?
            .join("applications/osu!lazer.desktop")),
        Paths::DesktopIcon => Ok(dirs::data_dir()
            .ok_or(error_message)?
            .join("osu-lazer-manager/icon.png")),
        Paths::Manager => Ok(dirs::data_dir()
            .ok_or(error_message)?
            .join("osu-lazer-manager")),
    }
}

pub fn get_path_as_str(target_file: Paths) -> Result<String> {
    Ok(get_path(target_file)?
        .to_str()
        .ok_or("Failed to convert path to string slice.")?
        .into())
}
