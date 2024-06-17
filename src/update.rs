use crate::install::install;

pub async fn update(do_not_make_default_version: bool, force_install: bool) -> anyhow::Result<()> {
    install(force_install, "latest", !do_not_make_default_version).await
}
