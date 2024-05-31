use crate::manager::js::obtain::Package;
use flate2::read::GzDecoder;
use simple_home_dir::home_dir;
use std::{
    fs::{create_dir_all, read_dir, remove_dir, remove_file, rename, write, File},
    path::{Path, MAIN_SEPARATOR_STR},
};
use tar::Archive;

pub async fn add_package_to_cache(package: Package) {
    let home_root = home_dir().unwrap();
    let cache_path = format!(
        "{}{os_separator}.qipi{os_separator}cache",
        home_root.display(),
        os_separator = MAIN_SEPARATOR_STR
    );

    if !Path::new(&cache_path).exists() {
        create_dir_all(&cache_path).unwrap();
    }

    let mut package_name: &str = package.name.as_str();
    let package_version: &str = package.version.as_str();

    #[allow(unused_assignments)]
    let mut package_replace = String::new();

    if package.name.contains("/") {
        package_replace = package.name.replace("/", "_");
        package_name = &package_replace;
    }

    let package_dir = format!(
        "{}{os_separator}{}{os_separator}{}",
        cache_path,
        package_name,
        package_version,
        os_separator = MAIN_SEPARATOR_STR
    );
    if !Path::new(&package_dir).exists() {
        create_dir_all(&package_dir).unwrap();
    }

    let tarball = reqwest::get(package.dist.tarball).await.unwrap();

    write(
        format!(
            "{}{os_separator}package.tar.gz",
            package_dir,
            os_separator = MAIN_SEPARATOR_STR
        ),
        tarball.bytes().await.unwrap(),
    )
    .unwrap();

    let tar_gz = File::open(format!(
        "{}{os_separator}package.tar.gz",
        package_dir,
        os_separator = MAIN_SEPARATOR_STR
    ))
    .unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&package_dir).unwrap();

    remove_file(format!(
        "{}{os_separator}package.tar.gz",
        package_dir,
        os_separator = MAIN_SEPARATOR_STR
    ))
    .unwrap();

    let package_subdir = format!(
        "{}{os_separator}package",
        package_dir,
        os_separator = MAIN_SEPARATOR_STR
    );
    if Path::new(&package_subdir).exists() {
        for entry in read_dir(&package_subdir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let filename = path.file_name().unwrap().to_str().unwrap();
            let new_path = path.to_owned();
            rename(
                new_path,
                format!(
                    "{}{os_separator}{}",
                    package_dir,
                    filename,
                    os_separator = MAIN_SEPARATOR_STR
                ),
            )
            .unwrap();
        }
        remove_dir(&package_subdir).unwrap();
    }
}

pub fn exists_package_in_cache(package: &Package) -> bool {
    let home_root = home_dir().unwrap();
    let cache_path = format!(
        "{}{os_separator}.qipi{os_separator}cache",
        home_root.display(),
        os_separator = MAIN_SEPARATOR_STR
    );

    if !Path::new(&cache_path).exists() {
        return false;
    }

    let package_dir = format!(
        "{}{os_separator}{}{os_separator}{}",
        cache_path,
        package.name,
        package.version,
        os_separator = MAIN_SEPARATOR_STR
    );

    if !Path::new(&package_dir).exists() {
        return false;
    }

    true
}

pub fn remove_package_from_cache(package: Package) {
    let home_root = home_dir().unwrap();
    let cache_path = format!(
        "{}{os_separator}.qipi{os_separator}cache",
        home_root.display(),
        os_separator = MAIN_SEPARATOR_STR
    );

    if !Path::new(&cache_path).exists() {
        return;
    }

    let package_dir = format!(
        "{}{os_separator}{}{os_separator}{}",
        cache_path,
        package.name,
        package.version,
        os_separator = MAIN_SEPARATOR_STR
    );

    if Path::new(&package_dir).exists() {
        remove_dir(&package_dir).unwrap();
    }
}
