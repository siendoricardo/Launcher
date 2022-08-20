use serde::{Deserialize, Serialize};

// ROOT
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifest {
    pub id: String,
    pub libraries: Vec<Library>,
    pub downloads: Downloads,
    #[serde(rename = "mainClass")]
    pub main_class: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Downloads {
    pub client: Artifact,
    pub server: Artifact,
}

// ASSETS

#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
    pub path: Option<String>,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Os {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rule {
    os: Os,
    action: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    pub name: String,
    pub downloads: LibraryDownloads,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryDownloads {
    pub artifact: Artifact,
}

// VERSIONS

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub r#type: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct VersionLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize)]
pub struct VersionsManifest {
    pub latest: VersionLatest,
    pub versions: Vec<Version>
}