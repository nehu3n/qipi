use anyhow::Result;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lockfile {
    pub lockfile_version: String,
    pub dependencies: HashMap<String, PackageInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageInfo {
    pub version: String,
    pub resolved: String,
    pub integrity: String,
    pub dependencies: Vec<String>,
}

pub const LOCKFILE_VERSION: &str = "1.0.0";

impl Lockfile {
    pub fn new() -> Lockfile {
        Lockfile {
            lockfile_version: LOCKFILE_VERSION.to_string(),
            dependencies: HashMap::new(),
        }
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_lockfile(destination: &str, packages: HashMap<String, PackageInfo>) -> Result<()> {
    let lockfile = Lockfile {
        dependencies: packages,
        ..Default::default()
    };

    let toml_string = toml::to_string(&lockfile)
        .map_err(|e| anyhow::anyhow!("Error serializing lockfile: {}", e))?;

    let comment = "# This (lock)file is automatically generated by Qipi. DO NOT EDIT.\n\n";
    let lockfile_content = format!("{}{}", comment, toml_string);

    let lockfile_path = format!("{}/qipi.toml", destination);
    let mut file = File::create(&lockfile_path)
        .map_err(|e| anyhow::anyhow!("Error creating lockfile: {}", e))?;

    file.write_all(lockfile_content.as_bytes())
        .map_err(|e| anyhow::anyhow!("Error writing lockfile: {}", e))?;

    Ok(())
}

pub fn load_lockfile(destination: &str) -> Result<Lockfile> {
    let lockfile_path = format!("{}/qipi.toml", destination);

    if !Path::new(&lockfile_path).exists() {
        return Err(anyhow::anyhow!("Lockfile not found."));
    }

    let content = std::fs::read_to_string(lockfile_path)
        .map_err(|e| anyhow::anyhow!("Error reading lockfile: {}", e))?;

    let lockfile: Lockfile =
        toml::from_str(&content).map_err(|e| anyhow::anyhow!("Error parsing lockfile: {}", e))?;

    Ok(lockfile)
}

pub fn update_lockfile(
    destination: &str,
    package_name: &str,
    package_info: PackageInfo,
) -> Result<()> {
    let mut lockfile = load_lockfile(destination).unwrap_or(Lockfile::new());

    lockfile
        .dependencies
        .insert(package_name.to_string(), package_info);

    generate_lockfile(destination, lockfile.dependencies)?;

    Ok(())
}
