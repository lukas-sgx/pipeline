use crate::{status, variables};
use indicatif::MultiProgress;
use quick_xml::de::from_str;
use serde::Deserialize;
use tokio::fs;
use colorized::{colorize_this, Colors};

#[derive(Deserialize)]
struct Schema {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "dir", default)]
    dirs: Vec<Dir>,
    #[serde(rename = "file", default)]
    files: Vec<File>,
}

#[derive(Deserialize)]
struct Dir {
    #[serde(rename = "@source")]
    source: Option<String>,
    #[serde(rename = "dir", default)]
    dirs: Vec<Dir>,
    #[serde(rename = "file", default)]
    files: Vec<File>,
}

#[derive(Deserialize)]
struct File {
    #[serde(rename = "@source")]
    source: Option<String>,
}

fn parse_xml(data: &str) -> Schema {
    from_str(data).unwrap()
}

async fn retrieve_map() -> Result<String, reqwest::Error> {
    let resp = reqwest::get(variables::MAP_XML_PATH).await?;
    let text = resp.text().await?;

    Ok(text)
}

async fn download_file(mp: &MultiProgress, version: &str, source: &String) -> anyhow::Result<()> {
    let pb = status::new_progress(mp, format!("Download {}", source));
    let path_file = format!("{}/{}/{}?raw=true", variables::STABLE_URL, version, source);
    let resp = reqwest::get(path_file).await?;
    let text = resp.text().await?;

    fs::write(source, text).await?;

    pb.finish_and_clear();

    Ok(())
}

async fn explore_dir(
    mp: &MultiProgress,
    dirs: Vec<Dir>,
    path: String,
    version: &str,
) -> anyhow::Result<()> {
    for dir in dirs {
        let new_path = format!("{}/{}", path, dir.source.unwrap_or_default());

        fs::create_dir_all(new_path.as_str()).await?;

        for file in dir.files {
            let source_path = format!("{}/{}", new_path, file.source.unwrap_or_default());
            download_file(mp, version, &source_path).await?;
        }
        Box::pin(explore_dir(mp, dir.dirs, new_path, version)).await?;
    }
    Ok(())
}

pub async fn download_template() -> anyhow::Result<()> {
    let map = retrieve_map().await?;
    let schema = parse_xml(&map);
    let mp = MultiProgress::new();
    let repo_pb = status::new_progress(&mp, "Setup repository...");
    let check = colorize_this("✔", Colors::GreenFg);
    let message_finish = format!("\r{} Setup repository complete !", check);

    for file in schema.files {
        download_file(&mp, &schema.version, &file.source.unwrap_or_default()).await?;
    }

    explore_dir(&mp, schema.dirs, String::from("."), schema.version.as_str()).await?;

    repo_pb.finish_with_message(message_finish);

    Ok(())
}
