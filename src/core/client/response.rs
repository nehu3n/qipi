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

#[derive(Clone, Default)]
pub enum Registry {
    #[default]
    NPM,
    JSR,
}

impl From<&str> for Registry {
    fn from(registry: &str) -> Self {
        match registry {
            "npm" => Self::NPM,
            "jsr" => Self::JSR,
            _ => Self::NPM,
        }
    }
}

#[derive(Default)]
pub struct Package {
    pub author: String,
    pub name: String,
    pub version: String,
    pub registry: Registry,
}

#[derive(Debug, Deserialize)]
pub struct Dist {
    pub tarball: String,
    pub integrity: String,
    pub shasum: String,
}
