use anyhow::{anyhow, Ok};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use osu_lazer_manager::{
    config::get_default_version,
    fetcher::{desktop::fetch_desktop, icon::fetch_icon},
};

pub async fn fix() -> anyhow::Result<()> {
    fix_desktop()
        .await
        .map_err(|e| anyhow!("Failed to fix desktop entry: {}", e))?;
    fix_icon()
        .await
        .map_err(|e| anyhow!("Failed to fix desktop icon: {}", e))?;

    Ok(())
}

async fn fix_desktop() -> anyhow::Result<()> {
    let progress_bar = create_progress_bar();

    fetch_desktop(
        &get_default_version()?,
        |size| progress_bar.set_length(size),
        |downloaded| progress_bar.set_position(downloaded),
    )
    .await?;

    println!("{}", "Installed desktop entry".green());

    Ok(())
}

async fn fix_icon() -> anyhow::Result<()> {
    let progress_bar = create_progress_bar();

    fetch_icon(
        |size| progress_bar.set_length(size),
        |downloaded| progress_bar.set_position(downloaded),
    )
    .await?;

    println!("{}", "Installed icon".green());

    Ok(())
}

fn create_progress_bar() -> ProgressBar {
    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
            .progress_chars("#>-"));
    progress_bar.set_message("Downloading file");

    progress_bar
}
