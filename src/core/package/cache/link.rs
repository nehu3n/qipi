use std::{
    env,
    fs::create_dir_all,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use symlink::symlink_auto;

use crate::config::get_cache_path;

pub fn link_package(name: &str, version: &str) {
    let package_cache_path = get_package_cache_path(name, version); // home/.qipi/cache/name/version/

    let project_root = Path::new("."); // proyect/
    let node_modules_dir = project_root.join("node_modules"); // proyect/node_modules
    let node_modules_qipi_dir = node_modules_dir.join(".qipi"); // proyect/node_modules/.qipi
    let package_versioned_dir = node_modules_qipi_dir.join(format!("{}@{}", name, version)); // proyect/node_modules/.qipi/name@version
    let package_node_modules_dir = package_versioned_dir.join("node_modules"); // proyect/node_modules/.qipi/name@version/node_modules

    create_dir_all(&package_node_modules_dir).unwrap(); // proyect/node_modules/.qipi/name@version/node_modules

    let cache_package_path = PathBuf::from(&package_cache_path); // home/.qipi/cache/name/version/
    let symlink_target = package_node_modules_dir.join(name); // proyect/node_modules/.qipi/name@version/node_modules/name

    if Path::new(&symlink_target).exists() {
        return;
    }

    symlink_auto(&cache_package_path, &symlink_target).unwrap(); // proyect/node_modules/.qipi/name@version/node_modules/name -> home/.qipi/cache/name/version

    let symlink_target_in_project = node_modules_dir.join(name); // proyect/node_modules/name
    if Path::new(&symlink_target_in_project).exists() {
        return;
    }

    symlink_auto(
        format!(
            "{}{MAIN_SEPARATOR_STR}node_modules{MAIN_SEPARATOR_STR}.qipi{MAIN_SEPARATOR_STR}{name}@{version}{MAIN_SEPARATOR_STR}node_modules{MAIN_SEPARATOR_STR}{name}",
            env::current_dir().unwrap().display()
        ),
        &symlink_target_in_project,
    )
    .unwrap() // proyect/node_modules/name -> proyect/node_modules/.qipi/name@version/node_modules/name
}

fn get_package_cache_path(name: &str, version: &str) -> String {
    let name = name.replace("/", "+").to_string();

    let cache_path = get_cache_path();
    format!("{cache_path}{MAIN_SEPARATOR_STR}{name}{MAIN_SEPARATOR_STR}{version}") // home/.qipi/cache/name/version
}
