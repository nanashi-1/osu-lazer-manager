use crate::{
    constants::REPOSITORY, paths::get_directory_path, utils::version::get_url_for_version,
};
use anyhow::anyhow;
use colored::Colorize;
use futures_util::StreamExt;
use std::{
    cmp::min,
    fs::File,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

trait Exists {
    fn exists(&self) -> bool;
}

impl Exists for PathBuf {
    fn exists(&self) -> bool {
        Path::exists(self)
    }
}

/// Fetch the appimage for the specific version.
pub async fn fetch_appimage(
    force_install: bool,
    specific_version: &str,
    init_callback: impl Fn(u64),
    progress_callback: impl Fn(u64),
) -> anyhow::Result<()> {
    let appimage_path = get_appimage_path(specific_version)?;

    handle_appimage_force_install(force_install, &appimage_path)?;

    let response = reqwest::get(get_url_for_version(specific_version)).await?;
    let size = response
        .content_length()
        .ok_or(anyhow!("Cannot get content length."))?;

    init_callback(size);

    let mut output_file = File::create(&appimage_path)?;
    output_file.set_permissions(PermissionsExt::from_mode(0o755))?;

    let mut download = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        output_file.write_all(&chunk)?;
        download = min(download + chunk.len() as u64, size);
        progress_callback(download);
    }

    Ok(())
}

pub fn get_appimage_path(specific_version: &str) -> anyhow::Result<PathBuf> {
    Ok(get_directory_path()?
        .join(REPOSITORY)
        .join(specific_version))
}

fn handle_appimage_force_install(
    force_install: bool,
    appimage_path: &impl Exists,
) -> anyhow::Result<()> {
    if !appimage_path.exists() {
        return Ok(());
    }

    if !force_install {
        return Err(anyhow::anyhow!(
            "File already exist! Consider using flag `--force`."
        ));
    }

    println!("{}", "Replacing existing file...".yellow());

    Ok(())
}

#[cfg(test)]
struct MockExists(bool);

#[cfg(test)]
impl Exists for MockExists {
    fn exists(&self) -> bool {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_appimage_force_install() {
        let mock_exists = MockExists(true);
        let test1 = handle_appimage_force_install(true, &mock_exists);
        let test2 = handle_appimage_force_install(false, &mock_exists);

        let mock_exists = MockExists(false);
        let test3 = handle_appimage_force_install(true, &mock_exists);
        let test4 = handle_appimage_force_install(false, &mock_exists);

        assert!(test1.is_ok());
        assert!(test2.is_err());
        assert!(test3.is_ok());
        assert!(test4.is_ok());
    }
}
