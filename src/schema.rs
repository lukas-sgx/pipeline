use crate::variables;
use quick_xml::de::from_str;
use serde::Deserialize;
use tokio::fs;

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

async fn download_file(version: &str, source: &String) -> anyhow::Result<()> {
    let path_file = format!("{}/{}/{}?raw=true", variables::STABLE_URL, version, source);
    let resp = reqwest::get(path_file).await?;

    let text = resp.text().await?;

    fs::write(source, text).await?;

    Ok(())
}

async fn explore_dir(dirs: Vec<Dir>, path: String, version: &str) -> anyhow::Result<()> {
    for dir in dirs {
        let new_path = format!("{}/{}", path, dir.source.unwrap_or_default());

        fs::create_dir_all(new_path.as_str()).await?;

        for file in dir.files {
            let source_path = format!("{}/{}", new_path, file.source.unwrap_or_default());
            download_file(version, &source_path).await?;
        }
        Box::pin(explore_dir(dir.dirs, new_path, version)).await?;
    }
    Ok(())
}

pub async fn download_template() -> anyhow::Result<()> {
    let map = retrieve_map().await?;
    let schema = parse_xml(&map);

    for file in schema.files {
        download_file(&schema.version, &file.source.unwrap_or_default()).await?;
    }

    explore_dir(schema.dirs, String::from("."), schema.version.as_str()).await?;
    Ok(())
}
