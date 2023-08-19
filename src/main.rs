use std::fs::{create_dir_all, metadata, remove_dir, remove_file, write};

use crate::error::Result;
use clap::{Parser, Subcommand};
use config::Config;
use fetcher::Fetcher;
use paths::{get_path, get_path_as_str};
use printer::Printer;
use scraper::{Html, Selector};

mod config;
mod error;
mod fetcher;
mod paths;
mod printer;

#[derive(Default, Clone)]
pub struct Context {
    pub force: bool,
    pub include_latest_version: bool,
    pub config: Config,
}

#[derive(Parser)]
#[command(version = "1", about = "Installs and Updates osu!lazer")]
struct Cli {
    #[command(subcommand)]
    command: Option<SubCommand>,
}

#[derive(Subcommand)]
enum SubCommand {
    #[command(about = "Install osu!lazer")]
    Install {
        #[clap(short, long, help = "Replace existing files from last install if any")]
        force: bool,
    },
    #[command(about = "Update osu!lazer")]
    Update {
        #[clap(short, long, help = "Replace existing files from last install if any")]
        force: bool,
    },
    #[command(about = "Check the version of the currently installed osu!lazer")]
    Version {
        #[clap(
            short = 'g',
            long,
            help = "Display both latest version and installed version"
        )]
        include_latest_version: bool,
    },
    #[command(about = "Uninstall osu!lazer")]
    Uninstall,
}

#[tokio::main]
async fn main() {
    if let Err(e) = app().await {
        e.to_string().print_as_error();
    }
}

async fn app() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load(get_path(paths::Paths::Config)?)?;

    match cli.command {
        Some(SubCommand::Install { force }) => {
            install(&mut Context {
                force,
                config,
                ..Default::default()
            })
            .await?
        }
        Some(SubCommand::Update { force }) => {
            update(&mut Context {
                force,
                config,
                ..Default::default()
            })
            .await?
        }
        Some(SubCommand::Version {
            include_latest_version,
        }) => {
            version(&Context {
                include_latest_version,
                config,
                ..Default::default()
            })
            .await?
        }
        Some(SubCommand::Uninstall) => uninstall().or(Err("Cannot uninstall nothing!"))?,
        None => todo!(),
    }

    Ok(())
}

async fn install(context: &mut Context) -> Result<()> {
    let fetcher = Fetcher::new(
        "https://github.com/ppy/osu/releases/latest/download/osu.AppImage",
        &get_path_as_str(paths::Paths::AppImage)?,
        context.to_owned(),
    );

    create_dir_all(get_path(paths::Paths::Manager)?)?;

    fetcher.fetch().await?;

    context.config.version = get_latest_version().await?;

    if desktop_file_exists()? && !context.force {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Desktop file already exists.",
        )));
    } else if desktop_file_exists()? && context.force {
        "Replacing existing desktop file...".print_as_warning();
    }

    let desktop_content = include_str!("../assets/osu!lazer.desktop")
        .replace("{appimage}", &get_path_as_str(paths::Paths::AppImage)?)
        .replace("{icon}", &get_path_as_str(paths::Paths::DesktopIcon)?);

    "Creating desktop file...".print();

    write(get_path(paths::Paths::DesktopFile)?, desktop_content)?;

    "Desktop file created!".print_as_success();

    if desktop_icon_exists()? && !context.force {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Desktop icon already exists.",
        )));
    } else if desktop_icon_exists()? && context.force {
        "Replacing existing desktop icon...".print_as_warning();
    }

    let icon_content = include_bytes!("../assets/icon.png");

    "Creating desktop icon...".print();

    write(get_path(paths::Paths::DesktopIcon)?, icon_content)?;

    "Desktop icon created!".print_as_success();

    context.config.save()?;

    "Installation Completed!".print_as_success();

    Ok(())
}

async fn update(context: &mut Context) -> Result<()> {
    let online_version = get_latest_version().await?;

    if context.config.version == online_version && !context.force {
        "Game is up to date!".print_as_warning();
        return Ok(());
    } else if context.config.version == online_version && context.force {
        "Replacing current installation...".print_as_warning();
    }

    context.force = true;

    let fetcher = Fetcher::new(
        "https://github.com/ppy/osu/releases/latest/download/osu.AppImage",
        &get_path_as_str(paths::Paths::AppImage)?,
        context.to_owned(),
    );
    fetcher.fetch().await?;

    context.config.version = get_latest_version().await?;
    context.config.save()?;

    "Update Complete!".print_as_success();

    Ok(())
}

async fn version(context: &Context) -> Result<()> {
    println!("Installed Version: {}", context.config.version);

    if context.include_latest_version {
        println!("Latest Version: {}", get_latest_version().await?)
    }

    Ok(())
}

fn uninstall() -> Result<()> {
    remove_file(get_path(paths::Paths::AppImage)?)?;
    remove_file(get_path(paths::Paths::Config)?)?;
    remove_file(get_path(paths::Paths::DesktopFile)?)?;
    remove_file(get_path(paths::Paths::DesktopIcon)?)?;

    remove_dir(get_path(paths::Paths::Manager)?)?;

    "Uninstall Complete".print_as_success();

    Ok(())
}

async fn get_latest_version() -> Result<String> {
    let body = reqwest::get("https://github.com/ppy/osu/releases/latest")
        .await?
        .text()
        .await?;

    let html = Html::parse_document(&body);
    let selector = Selector::parse(r#"h1.d-inline.mr-3[data-view-component="true"]"#)?;

    Ok(html
        .select(&selector)
        .next()
        .ok_or("No matches!")?
        .inner_html())
}

fn desktop_file_exists() -> Result<bool> {
    Ok(metadata(get_path(paths::Paths::DesktopFile)?).is_ok())
}

fn desktop_icon_exists() -> Result<bool> {
    Ok(metadata(get_path(paths::Paths::DesktopIcon)?).is_ok())
}
