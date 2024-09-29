use std::{
    fs::File,
    io::{BufReader, Write},
    path::{Path, MAIN_SEPARATOR_STR},
};

use flate2::bufread::GzDecoder;
use tar::Archive;

pub fn download_tarball(
    bytes: Vec<u8>,
    destination: &str,
    name: &str,
    version: &str,
) -> Result<(), String> {
    let cache_path = format!("{destination}{MAIN_SEPARATOR_STR}{name}@{version}");
    let tarball_path = format!("{cache_path}.tar.gz");

    let mut temp_file =
        File::create(&tarball_path).map_err(|e| format!("Error creating temp file: {}", e))?;
    temp_file
        .write_all(&bytes)
        .map_err(|e| format!("Error writing to temp file: {}", e))?;

    let file = File::open(&tarball_path).map_err(|e| format!("Error opening temp file: {}", e))?;
    let buf_reader = BufReader::new(file);
    let decoder = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(decoder);

    let dest_path = Path::new(&cache_path);
    if !dest_path.exists() {
        std::fs::create_dir_all(dest_path)
            .map_err(|e| format!("Error creating destination directory: {}", e))?;
    }

    archive
        .unpack(dest_path)
        .map_err(|e| format!("Error extracting tarball: {}", e))?;

    std::fs::remove_file(tarball_path).map_err(|e| format!("Error deleting temp file: {}", e))?;

    Ok(())
}
