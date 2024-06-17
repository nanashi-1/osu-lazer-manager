use super::{appimage::get_appimage_path, icon::get_icon_path};
use crate::{
    constants::{DESKTOP, DESKTOP_URL},
    paths::{get_application_path, get_directory_path},
};
use anyhow::anyhow;
use futures_util::StreamExt;
use std::{
    cmp::min,
    fs::{read_to_string, write, File},
    io::Write,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};

pub async fn fetch_desktop(
    specific_version: &str,
    init_callback: impl Fn(u64),
    progress_callback: impl Fn(u64),
) -> anyhow::Result<()> {
    let desktop_cache_path = get_desktop_cache_path()?;

    let response = reqwest::get(DESKTOP_URL).await?;
    let size = response
        .content_length()
        .ok_or(anyhow!("Cannot get content length."))?;

    init_callback(size);

    let mut output_file = File::create(&desktop_cache_path)?;
    output_file.set_permissions(PermissionsExt::from_mode(0o755))?;

    let mut download = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        output_file.write_all(&chunk)?;
        download = min(download + chunk.len() as u64, size);
        progress_callback(download);
    }

    let desktop_path = get_desktop_path()?;
    let desktop_template = read_to_string(desktop_cache_path)?;

    let output_desktop = process_template(
        &desktop_template,
        get_icon_path()?.to_str().unwrap(),
        get_appimage_path(specific_version)?.to_str().unwrap(),
    );

    write(desktop_path, output_desktop)?;

    Ok(())
}

pub fn get_desktop_path() -> anyhow::Result<PathBuf> {
    Ok(get_application_path()?.join(DESKTOP))
}

pub fn get_desktop_cache_path() -> anyhow::Result<PathBuf> {
    Ok(get_directory_path()?.join(DESKTOP))
}

pub fn process_template(template: &str, icon_path: &str, appimage_path: &str) -> String {
    template
        .replace("{icon}", icon_path)
        .replace("{appimage}", appimage_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_template() {
        let template = "{icon}\n{appimage}";
        let icon_path = "/path/to/icon.png";
        let appimage_path = "/path/to/appimage";
        let output = process_template(template, icon_path, appimage_path);
        assert_eq!(output, "/path/to/icon.png\n/path/to/appimage");
    }
}
