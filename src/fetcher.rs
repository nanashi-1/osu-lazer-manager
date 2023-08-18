use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{error::Result, printer::Printer, Context};
use std::{
    cmp::min,
    fs::{metadata, File},
    io::Write,
    os::unix::prelude::PermissionsExt,
};

pub struct Fetcher {
    url: String,
    path: String,
    context: Context,
}

impl Fetcher {
    pub fn new(url: &str, path: &str, context: Context) -> Self {
        Self {
            url: url.into(),
            path: path.into(),
            context,
        }
    }

    pub async fn fetch(&self) -> Result<()> {
        if self.file_exists()? && !self.context.force {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "File already exist! Consider using flag `--force`.",
            )));
        } else if self.file_exists()? && self.context.force {
            "Replacing existing file...".print_as_warning();
        }

        let res = reqwest::get(&self.url).await?;
        let size = res.content_length().ok_or("Cannot get content length.")?;

        let pb = ProgressBar::new(size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
            .progress_chars("#>-"));
        pb.set_message("Downloading file");

        let mut out = File::create(&self.path)?;
        out.set_permissions(PermissionsExt::from_mode(0o755))?;

        let mut download: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(v) = stream.next().await {
            let chunk = v?;
            out.write_all(&chunk)?;
            download = min(download + chunk.len() as u64, size);

            pb.set_position(download)
        }

        "File downloaded".print_as_success();

        Ok(())
    }

    fn file_exists(&self) -> Result<bool> {
        Ok(metadata(&self.path).is_ok())
    }
}
