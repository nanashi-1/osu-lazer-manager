use crate::constants::DOWNLOAD_URL;
use anyhow::Ok;
use colored::Colorize;
use scraper::{Html, Selector};
use serde::Deserialize;

#[derive(Deserialize)]
struct LatestReleaseJsonResponse {
    name: String,
}

/// Get the latest version of osu from the GitHub release page.
pub async fn get_latest_version() -> anyhow::Result<String> {
    // let body = reqwest::get("https://github.com/ppy/osu/releases/latest")
    //     .await?
    //     .text()
    //     .await?;

    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let json_str = client
        .get("https://api.github.com/repos/ppy/osu/releases/latest")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2026-03-10")
        .send()
        .await?
        .text()
        .await?;

    Ok(get_version_from_api(&json_str))
}

/// Get the download URL for the specific version.
pub fn get_url_for_version(specific_version: &str) -> String {
    DOWNLOAD_URL.replace("{version}", specific_version)
}

fn get_version_from_api(json_str: &str) -> String {
    let parsed_json: LatestReleaseJsonResponse = serde_json::from_str(json_str).unwrap();

    parsed_json.name
}

/// DEPRECATED
fn _get_version_from_body(body: &str) -> String {
    let html = Html::parse_document(body);
    let selector = Selector::parse(r#"h1.d-inline.mr-3[data-view-component="true"]"#).unwrap();

    html.select(&selector).next().unwrap_or_else(|| { panic!("{}", "Could not find version. Please verify this version exists, update this tool, or report this issue here: https://github.com/nanashi-1/osu-lazer-manager/issues/new".red().to_string()) }).inner_html()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_from_body() -> anyhow::Result<()> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("OsuLazerManagerTest/1")
            .build()?;

        let json_str = client
            .get("https://api.github.com/repos/nanashi-1/osu-lazer-manager/releases/161460233")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2026-03-10")
            .send()?
            .text()?;

        assert_eq!(get_version_from_api(&json_str), "v1.0.1");

        Ok(())
    }
}
