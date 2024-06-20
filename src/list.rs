use anyhow::Ok;
use colored::Colorize;
use osu_lazer_manager::{constants::REPOSITORY, paths::get_directory_path};

pub fn list() -> anyhow::Result<()> {
    let repository = get_directory_path()
        .map_err(|e| anyhow::anyhow!("Failed to get osu-lazer-manager data directory: {}", e))?
        .join(REPOSITORY);

    if !repository.exists() {
        return Err(anyhow::anyhow!("No versions installed"));
    }

    if repository.read_dir()?.count() == 0 {
        return Err(anyhow::anyhow!("No versions installed"));
    }

    println!("{}", "Installed versions:".green());

    repository
        .read_dir()
        .map_err(|e| anyhow::anyhow!("Failed to read directory: {}", e))?
        .for_each(|entry| {
            println!("{}", entry.unwrap().file_name().into_string().unwrap());
        });

    Ok(())
}
