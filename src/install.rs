use anyhow::{anyhow, Ok};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use osu_lazer_manager::{
    config::{get_default_version, set_default_version},
    constants::REPOSITORY,
    fetcher::{
        appimage::{fetch_appimage, get_appimage_path},
        desktop::{fetch_desktop, get_desktop_cache_path, get_desktop_path, process_template},
        icon::{fetch_icon, get_icon_path},
    },
    paths::get_directory_path,
    utils::{directories::create_missing_directories, version::get_latest_version},
};
use std::{
    fs::{read_dir, read_to_string, write},
    path::PathBuf,
};

trait OnlyContainsOne {
    fn only_contains_one(&self) -> anyhow::Result<bool>;
}

impl OnlyContainsOne for PathBuf {
    fn only_contains_one(&self) -> anyhow::Result<bool> {
        Ok(read_dir(self)?.count() == 1)
    }
}

pub async fn install(
    force_install: bool,
    version: &str,
    make_default_version: bool,
) -> anyhow::Result<()> {
    println!("Installing osu!lazer {} version...", version);

    create_missing_directories()?;

    let specific_version = match version {
        "latest" => get_latest_version().await?,
        _ => version.to_string(),
    };

    get_appimage(force_install, &specific_version, make_default_version).await?;
    get_icon().await?;
    get_desktop(&specific_version, make_default_version).await?;

    Ok(())
}

async fn get_appimage(
    force_install: bool,
    specific_version: &str,
    make_default_version: bool,
) -> anyhow::Result<()> {
    let progress_bar = create_progress_bar();

    fetch_appimage(
        force_install,
        specific_version,
        |size| progress_bar.set_length(size),
        |downloaded| progress_bar.set_position(downloaded),
    )
    .await?;

    println!("{}", "Installed AppImage".green());

    let repository_path = get_directory_path()?.join(REPOSITORY);

    if should_set_default_version(make_default_version, &repository_path)? {
        println!("Setting as default version");
        set_default_version(specific_version)?;
    }

    Ok(())
}

async fn get_icon() -> anyhow::Result<()> {
    let progress_bar = create_progress_bar();

    if fetch_icon(
        |size| progress_bar.set_length(size),
        |downloaded| progress_bar.set_position(downloaded),
    )
    .await
    .is_err()
    {
        match get_icon_path()?.exists() {
            true => println!("Will use existing icon."),
            false => return Err(anyhow!("Failed to download icon.")),
        }
    }

    println!("{}", "Installed icon".green());

    Ok(())
}

async fn get_desktop(specific_version: &str, make_default_version: bool) -> anyhow::Result<()> {
    let progress_bar = create_progress_bar();

    let repository_path = get_directory_path()?.join(REPOSITORY);

    let version = match should_set_default_version(make_default_version, &repository_path)? {
        true => specific_version.to_owned(),
        false => get_default_version()?,
    };

    if fetch_desktop(
        &version,
        |size| progress_bar.set_length(size),
        |downloaded| progress_bar.set_position(downloaded),
    )
    .await
    .is_err()
    {
        let desktop_template = read_to_string(get_desktop_cache_path()?)?;

        let output_desktop = process_template(
            &desktop_template,
            get_icon_path()?.to_str().unwrap(),
            get_appimage_path(&version)?.to_str().unwrap(),
        );

        write(get_desktop_path()?, output_desktop)?;
    }

    println!("{}", "Installed desktop entry".green());

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

fn should_set_default_version(
    make_default_version: bool,
    repository_path: &impl OnlyContainsOne,
) -> anyhow::Result<bool> {
    Ok(make_default_version || repository_path.only_contains_one()?)
}

#[cfg(test)]
struct MockOnlyContainsOne(bool);

#[cfg(test)]
impl OnlyContainsOne for MockOnlyContainsOne {
    fn only_contains_one(&self) -> anyhow::Result<bool> {
        Ok(self.0)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_should_set_default_version() -> anyhow::Result<()> {
        let mock_only_contains_one = MockOnlyContainsOne(true);
        let test1 = should_set_default_version(true, &mock_only_contains_one)?;
        let test2 = should_set_default_version(false, &mock_only_contains_one)?;

        let mock_only_contains_one = MockOnlyContainsOne(false);
        let test3 = should_set_default_version(true, &mock_only_contains_one)?;
        let test4 = should_set_default_version(false, &mock_only_contains_one)?;

        assert!(test1);
        assert!(test2);
        assert!(test3);
        assert!(!test4);

        Ok(())
    }

    #[tokio::test]
    async fn test_install() -> anyhow::Result<()> {
        install(false, "latest", false).await?;

        Ok(())
    }
}
