use crate::{
    constants::{ICON, ICON_URL},
    paths::get_directory_path,
};
use anyhow::anyhow;
use futures_util::StreamExt;
use std::{cmp::min, fs::File, io::Write, os::unix::fs::PermissionsExt, path::PathBuf};

pub async fn fetch_icon(
    init_callback: impl Fn(u64),
    progress_callback: impl Fn(u64),
) -> anyhow::Result<()> {
    let icon_path = get_icon_path()?;

    let response = reqwest::get(ICON_URL).await?;
    let size = response
        .content_length()
        .ok_or(anyhow!("Cannot get content length."))?;

    init_callback(size);

    let mut output_file = File::create(&icon_path)?;
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

pub fn get_icon_path() -> anyhow::Result<PathBuf> {
    Ok(get_directory_path()?.join(ICON))
}
