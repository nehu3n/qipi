use std::fs::create_dir;
use std::path::{Path, MAIN_SEPARATOR_STR};

use simple_home_dir::home_dir;

use crate::manager::js::obtain::Package;

pub fn link_package(package: &Package) {
    let mut package_name: &str = package.name.as_str();
    let package_version: &str = package.version.as_str();

    #[allow(unused_assignments)]
    let mut package_replace = String::new();

    if package.name.contains("/") {
        package_replace = package.name.replace("/", "_");
        package_name = &package_replace;
    }

    let home_root = home_dir().unwrap();
    let package_dir = format!(
        "{}{os_separator}.qipi{os_separator}cache{os_separator}{}{os_separator}{}",
        home_root.display(),
        package_name,
        package_version,
        os_separator = MAIN_SEPARATOR_STR
    );
    if !Path::new(&package_dir).exists() {
        return;
    }

    let node_modules_dir = format!(
        ".{os_separator}node_modules",
        os_separator = MAIN_SEPARATOR_STR
    )
    .to_string();

    if !Path::new(&node_modules_dir).exists() {
        create_dir(&node_modules_dir).unwrap();
    }

    let dst_path = format!(
        "{}{os_separator}{}",
        node_modules_dir,
        package_name,
        os_separator = MAIN_SEPARATOR_STR
    );
    if Path::new(&dst_path).exists() {
        return;
    }

    println!("{}", dst_path);

    symlink::symlink_dir(&package_dir, dst_path).unwrap();
}
