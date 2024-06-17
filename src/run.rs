use osu_lazer_manager::{
    config::get_default_version, constants::REPOSITORY, paths::get_directory_path,
};
use std::{os::unix::process::CommandExt, process::Command};

pub fn run(version: &str) -> anyhow::Result<()> {
    let version = match version {
        "default" => get_default_version()?,
        _ => version.to_owned(),
    };

    println!("Running osu!lazer {}", version);

    Command::new(get_directory_path()?.join(REPOSITORY).join(version)).exec();

    Ok(())
}
