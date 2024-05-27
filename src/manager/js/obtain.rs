use std::collections::HashMap;

use rkyv::{Archive, Deserialize, Serialize};

#[archive(check_bytes)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, serde::Deserialize, Clone)]
pub struct PackageRepository {
    pub r#type: String,
    pub url: String,
}

#[archive(check_bytes)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, serde::Deserialize, Clone)]
pub struct PackageDist {
    pub integrity: String,
    pub shasum: String,
    pub tarball: String,
}

#[archive(check_bytes)]
#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, serde::Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub repository: PackageRepository,
    #[serde(default)]
    pub main: String,
    #[serde(default)]
    pub devDependencies: HashMap<String, String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub gitHead: String,
    pub dist: PackageDist,
}

pub async fn obtain_package(name: &str, version: &str) -> Result<Package, reqwest::Error> {
    let resp = reqwest::get(format!("https://registry.npmjs.org/{}/{}", name, version))
        .await?
        .json::<Package>()
        .await?;

    return Ok(resp);
}
