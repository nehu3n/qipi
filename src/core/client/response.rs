use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct NPMPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dist: Dist,
    pub dependencies: Option<HashMap<String, String>>,
    pub devDependencies: Option<HashMap<String, String>>,
    pub peerDependencies: Option<HashMap<String, String>>,
}

pub struct Package {
    pub author: String,
    pub name: String,
    pub version: String,
    pub registry: String,
}

#[derive(Debug, Deserialize)]
pub struct Dist {
    pub tarball: String,
    pub integrity: String,
    pub shasum: String,
}
