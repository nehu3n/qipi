use std::{
    fs::{create_dir_all, remove_file, rename, File},
    io::{BufReader, Write},
    path::{Path, MAIN_SEPARATOR_STR},
};

use anyhow::Result;
use flate2::bufread::GzDecoder;
use tar::Archive;

pub fn download_tarball(
    bytes: Vec<u8>,
    destination: &str,
    name: &str,
    version: &str,
) -> Result<()> {
    let name = normalize_package_name(name);

    let cache_path = format!("{destination}{MAIN_SEPARATOR_STR}{name}{MAIN_SEPARATOR_STR}");

    if !Path::new(&cache_path).exists() {
        create_dir_all(&cache_path)
            .map_err(|e| anyhow::anyhow!("Error creating cache directory: {}", e))?;
    }

    let tarball_path = format!("{cache_path}.tar.gz");

    let mut temp_file = File::create(&tarball_path)
        .map_err(|e| anyhow::anyhow!("Error creating temp file: {}", e))?;
    temp_file
        .write_all(&bytes)
        .map_err(|e| anyhow::anyhow!("Error writing to temp file: {}", e))?;

    let file =
        File::open(&tarball_path).map_err(|e| anyhow::anyhow!("Error opening temp file: {}", e))?;
    let buf_reader = BufReader::new(file);
    let decoder = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(decoder);

    let dest_path = Path::new(&cache_path);
    if !dest_path.exists() {
        create_dir_all(dest_path)
            .map_err(|e| anyhow::anyhow!("Error creating destination directory: {}", e))?;
    }

    archive
        .unpack(dest_path)
        .map_err(|e| anyhow::anyhow!("Error extracting tarball: {}", e))?;

    remove_file(tarball_path).map_err(|e| anyhow::anyhow!("Error deleting temp file: {}", e))?;

    let package_path = format!("{cache_path}{MAIN_SEPARATOR_STR}package");

    let new_package_path = format!("{cache_path}{MAIN_SEPARATOR_STR}{version}");
    if Path::new(&package_path).exists() {
        rename(package_path, new_package_path)
            .map_err(|e| anyhow::anyhow!("Error renaming package dir: {}", e))?;
    }

    Ok(())
}

pub fn has_tarball_in_cache(destination: &str, name: &str, version: &str) -> bool {
    let cache_path = format!("{destination}{MAIN_SEPARATOR_STR}{name}@{version}");
    
    Path::new(&cache_path).exists()
}

fn normalize_package_name(name: &str) -> String {
    name.replace("/", "+").to_string()
}
