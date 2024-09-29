use std::path::PathBuf;

use simple_home_dir::home_dir;

pub fn get_qipi_root_path() -> String {
    let home = home_dir().unwrap();
    let path = home.join(".qipi");

    if !path.exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path.to_str().unwrap().to_string()
}

pub fn get_config_path() -> String {
    let root = get_qipi_root_path();
    let path = PathBuf::new().join(&root).join("config.toml");

    if !path.exists() {
        std::fs::File::create(&path).unwrap();
    }

    path.to_str().unwrap().to_string()
}

pub fn get_cache_path() -> String {
    let root = get_qipi_root_path();
    let path = PathBuf::new().join(&root).join("cache");

    if !path.exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path.to_str().unwrap().to_string()
}
