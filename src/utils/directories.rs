use crate::{
    constants::REPOSITORY,
    paths::{get_application_path, get_directory_path},
};

pub fn create_missing_directories() -> anyhow::Result<()> {
    let directory_path = get_directory_path()?;
    let repository_path = directory_path.join(REPOSITORY);
    let application_path = get_application_path()?;

    if !directory_path.exists() {
        std::fs::create_dir_all(&directory_path)?;
    }

    if !repository_path.exists() {
        std::fs::create_dir_all(&repository_path)?;
    }

    if !application_path.exists() {
        std::fs::create_dir_all(&application_path)?;
    }

    Ok(())
}
