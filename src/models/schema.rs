use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Schema {
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "dir", default)]
    pub dirs: Vec<Dir>,
    #[serde(rename = "file", default)]
    pub files: Vec<File>,
}

#[derive(Deserialize)]
pub struct Dir {
    #[serde(rename = "@source")]
    pub source: Option<String>,
    #[serde(rename = "dir", default)]
    pub dirs: Vec<Dir>,
    #[serde(rename = "file", default)]
    pub files: Vec<File>,
}

#[derive(Deserialize)]
pub struct File {
    #[serde(rename = "@source")]
    pub source: Option<String>,
}

pub fn parse_xml(data: &str) -> Schema {
    from_str(data).unwrap()
}