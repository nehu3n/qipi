use std::fs::create_dir_all;
use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};

use simple_home_dir::home_dir;

use crate::manager::js::obtain::Package;

use std::env;
use symlink::symlink_dir;

pub fn link_package(package: &Package) {
    let package_name: &str = package.name.as_str();
    let package_version: &str = package.version.as_str();

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

    let project_root = Path::new(".");
    let node_modules_dir = project_root.join("node_modules");
    let node_modules_qipi_dir = node_modules_dir.join(".qipi");
    let package_versioned_dir = node_modules_qipi_dir.join(format!("{}@{}", package_name, package_version));
    let package_node_modules_dir = package_versioned_dir.join("node_modules");

    create_dir_all(&package_node_modules_dir)
        .unwrap();

    let cache_package_path = PathBuf::from(&package_dir);
    let symlink_target = package_node_modules_dir.join(package_name);
    symlink_dir(&cache_package_path, &symlink_target).unwrap();

    let symlink_target_in_project = node_modules_dir.join(package_name);
    symlink_dir(
        format!(
            "{}/node_modules/.qipi/{package_name}@{package_version}/node_modules/{package_name}",
            env::current_dir().unwrap().display()
        ),
        &symlink_target_in_project,
    )
    .unwrap()
}
