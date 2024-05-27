use crate::manager::js::obtain::Package;
use flate2::read::GzDecoder;
use simple_home_dir::home_dir;
use std::{
    fs::{create_dir_all, read_dir, remove_dir, remove_file, rename, write, File},
    path::Path,
};
use tar::Archive;

pub async fn add_package_to_cache(package: Package) {
    let home_root = home_dir().unwrap();
    let cache_path = format!("{}/.qipi/cache", home_root.display());

    if !Path::new(&cache_path).exists() {
        create_dir_all(&cache_path).unwrap();
    }

    let package_dir = format!("{}/{}/{}", cache_path, package.name, package.version);
    if !Path::new(&package_dir).exists() {
        create_dir_all(&package_dir).unwrap();
    }

    let tarball = reqwest::get(package.dist.tarball).await.unwrap();

    write(
        format!("{}/package.tar.gz", package_dir),
        tarball.bytes().await.unwrap(),
    )
    .unwrap();

    let tar_gz = File::open(format!("{}/package.tar.gz", package_dir)).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&package_dir).unwrap();

    remove_file(format!("{}/package.tar.gz", package_dir)).unwrap();

    let package_subdir = format!("{}/package", package_dir);
    if Path::new(&package_subdir).exists() {
        for entry in read_dir(&package_subdir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let new_path = path.to_owned();
            rename(new_path, format!("{}/{}", package_dir, filename)).unwrap();
        }
        remove_dir(&package_subdir).unwrap();
    }
}

pub fn exists_package_in_cache(package: &Package) -> bool {
    let home_root = home_dir().unwrap();
    let cache_path = format!("{}/.qipi/cache", home_root.display());

    if !Path::new(&cache_path).exists() {
        return false;
    }

    let package_dir = format!("{}/{}/{}", cache_path, package.name, package.version);

    if !Path::new(&package_dir).exists() {
        return false;
    }

    true
}

pub fn remove_package_from_cache(package: Package) {
    let home_root = home_dir().unwrap();
    let cache_path = format!("{}/.qipi/cache", home_root.display());

    if !Path::new(&cache_path).exists() {
        return;
    }

    let package_dir = format!("{}/{}/{}", cache_path, package.name, package.version);

    if Path::new(&package_dir).exists() {
        remove_dir(&package_dir).unwrap();
    }
}
