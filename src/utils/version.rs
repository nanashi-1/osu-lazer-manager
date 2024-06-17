use crate::constants::DOWNLOAD_URL;
use anyhow::Ok;
use colored::Colorize;
use scraper::{Html, Selector};

/// Get the latest version of osu from the GitHub release page.
pub async fn get_latest_version() -> anyhow::Result<String> {
    let body = reqwest::get("https://github.com/ppy/osu/releases/latest")
        .await?
        .text()
        .await?;

    Ok(get_version_from_body(&body))
}

/// Get the download URL for the specific version.
pub fn get_url_for_version(specific_version: &str) -> String {
    DOWNLOAD_URL.replace("{version}", specific_version)
}

fn get_version_from_body(body: &str) -> String {
    let html = Html::parse_document(body);
    let selector = Selector::parse(r#"h1.d-inline.mr-3[data-view-component="true"]"#).unwrap();

    html.select(&selector).next().unwrap_or_else(|| { panic!("{}", "Could not find version. Please verify this version exists, update this tool, or report this issue here: https://github.com/nanashi-1/osu-lazer-manager/issues/new".red().to_string()) }).inner_html()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_from_body() -> anyhow::Result<()> {
        let body =
            reqwest::blocking::get("https://github.com/ppy/osu/releases/tag/2024.521.2")?.text()?;
        assert_eq!(get_version_from_body(&body), "2024.521.2");

        Ok(())
    }
}
