use std::{collections::HashMap, fs::read_to_string};

use serde::Deserialize;
use yaml_rust2::YamlLoader;

#[derive(Debug, Deserialize, Default)]
struct PackageNPM {
    #[serde(default)]
    name: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    license: Option<String>,
    #[serde(default)]
    dependencies: Option<HashMap<String, String>>,
    #[serde(default)]
    resolved: Option<String>,
    #[serde(default)]
    integrity: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct LockfileNPM {
    name: String,
    version: String,
    lockfileVersion: u8,
    requires: bool,
    #[serde(default)]
    packages: HashMap<String, PackageNPM>,
}

pub fn cross_lockfile_npm_parser() {
    let lockfile = read_to_string("package-lock.json").unwrap();

    let lockfile = serde_json::from_str::<LockfileNPM>(&lockfile).unwrap();

    println!("{:#?}", lockfile);
}

pub fn cross_lockfile_pnpm_parser() {
    let lockfile = read_to_string("pnpm-lock.yaml").unwrap();

    let lockfile = YamlLoader::load_from_str(&lockfile).unwrap();

    println!("{:#?}", lockfile[0].as_str().unwrap());
}
