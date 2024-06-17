use anyhow::Ok;
use osu_lazer_manager::{
    config::get_default_version, constants::REPOSITORY, fetcher::appimage::get_appimage_path,
    paths::get_directory_path, utils::version::get_latest_version,
};

pub async fn uninstall(version: &str) -> anyhow::Result<()> {
    let version = match version {
        "latest" => get_latest_version().await?,
        _ => version.to_string(),
    };

    let version = match version.as_str() {
        "latest" => get_latest_version().await?,
        _ => version.to_string(),
    };

    match version.as_str() {
        "all" => {
            let reposistory = get_directory_path()?.join(REPOSITORY);

            std::fs::remove_dir_all(reposistory)?;
        }
        "default" => {
            let appimage_path = get_appimage_path(&get_default_version()?)?;

            std::fs::remove_file(appimage_path)?;
        }
        "except" => {
            let reposistory = get_directory_path()?.join(REPOSITORY);
            let default = get_default_version()?;

            for file in reposistory.read_dir()? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                if file_name != default.as_str() {
                    std::fs::remove_file(&path)?;
                }
            }
        }
        _ => {
            let appimage_path = get_appimage_path(&version)?;

            std::fs::remove_file(appimage_path)?;
        }
    };

    Ok(())
}
