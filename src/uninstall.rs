use anyhow::{anyhow, Ok};
use osu_lazer_manager::{
    config::get_default_version, constants::REPOSITORY, fetcher::appimage::get_appimage_path,
    paths::get_directory_path, utils::version::get_latest_version,
};

pub async fn uninstall(version: &str) -> anyhow::Result<()> {
    let version = match version {
        "latest" => get_latest_version()
            .await
            .map_err(|e| anyhow!("Failed to get latest version: {}", e))?,
        _ => version.to_string(),
    };

    let version = match version.as_str() {
        "latest" => get_latest_version()
            .await
            .map_err(|e| anyhow!("Failed to get latest version: {}", e))?,
        _ => version.to_string(),
    };

    match version.as_str() {
        "all" => {
            let reposistory = get_directory_path()
                .map_err(|e| anyhow!("Failed to get osu-lazer-manager data directory: {}", e))?
                .join(REPOSITORY);

            std::fs::remove_dir_all(reposistory)
                .map_err(|e| anyhow!("Failed to remove osu-lazer-manager data directory: {}", e))?;
        }
        "default" => {
            let appimage_path = get_appimage_path(
                &get_default_version()
                    .map_err(|e| anyhow!("Failed to get default version: {}", e))?,
            )
            .map_err(|e| anyhow!("Failed to get appimage path: {}", e))?;

            std::fs::remove_file(appimage_path)
                .map_err(|e| anyhow!("Failed to remove appimage: {}", e))?;
        }
        "except" => {
            let reposistory = get_directory_path()
                .map_err(|e| anyhow!("Failed to get osu-lazer-manager data directory: {}", e))?
                .join(REPOSITORY);
            let default = get_default_version()
                .map_err(|e| anyhow!("Failed to get default version: {}", e))?;

            for file in reposistory
                .read_dir()
                .map_err(|e| anyhow!("Failed to read osu-lazer-manager data directory: {}", e))?
            {
                let file = file.map_err(|e| {
                    anyhow!("Failed to read osu-lazer-manager data directory: {}", e)
                })?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name != default.as_str() {
                    std::fs::remove_file(&path)
                        .map_err(|e| anyhow!("Failed to remove file: {}", e))?;
                }
            }
        }
        _ => {
            let appimage_path = get_appimage_path(&version)
                .map_err(|e| anyhow!("Failed to get appimage path: {}", e))?;

            std::fs::remove_file(appimage_path)
                .map_err(|e| anyhow!("Failed to remove appimage: {}", e))?;
        }
    };

    Ok(())
}
