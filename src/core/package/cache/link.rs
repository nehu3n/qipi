use std::{
    collections::HashMap,
    env,
    fs::create_dir_all,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use regex::Regex;
use symlink::symlink_auto;

use crate::{
    config::get_cache_path,
    core::client::{
        http::{get_package, get_tarball},
        response::Package,
    },
};

use super::tarball::download_tarball;

pub fn link_package(name: &str, version: &str) {
    println!("linking {}@{}", name, version);

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

pub async fn link_dependency(
    package_name: &str,
    version: &str,
    dependencies: &[(String, String)],
    already_resolved: &mut HashMap<String, String>,
) {
    let project_root = Path::new("."); // proyect/
    let node_modules_dir = project_root.join("node_modules"); // proyect/node_modules/
    let node_modules_qipi_dir = node_modules_dir.join(".qipi"); // proyect/node_modules/.qipi/
    let package_versioned_dir = node_modules_qipi_dir.join(format!("{}@{}", package_name, version)); // proyect/node_modules/.qipi/package_name@version/
    let package_node_modules_dir = package_versioned_dir.join("node_modules"); // proyect/node_modules/.qipi/package_name@version/node_modules/

    create_dir_all(&package_node_modules_dir).unwrap(); // proyect/node_modules/.qipi/package_name@version/node_modules/

    for (dep_name, dep_version) in dependencies {
        let dep_version = &parse_version(&dep_version).unwrap();

        if let Some(existing_version) = already_resolved.get(dep_name) {
            if existing_version == dep_version {
                continue;
            } else {
                let unique_symlink_target =
                    node_modules_qipi_dir.join(format!("{}@{}", dep_name, dep_version));

                let cache_dep_path = get_package_cache_path(dep_name, dep_version); // home/.qipi/cache/dep_name/dep_version/

                if !Path::new(&cache_dep_path).exists() {
                    download_tarball(
                        get_tarball(
                            get_package(Package {
                                name: dep_name.to_string(),
                                version: dep_version.to_string(),
                                author: "".to_string(),
                                registry: "npm".to_string(),
                            })
                            .await
                            .unwrap()
                            .dist
                            .tarball,
                        )
                        .await
                        .unwrap(),
                        &get_cache_path(),
                        dep_name,
                        dep_version,
                    )
                    .unwrap();
                }

                if !Path::new(&unique_symlink_target).exists() {
                    symlink_auto(&cache_dep_path, &unique_symlink_target).unwrap_or_else(|e| {
                        eprintln!("Error creating symlink {}: {:?}", dep_name, e);
                    });
                }

                continue;
            }
        }

        let cache_dep_path = get_package_cache_path(dep_name, dep_version); // home/.qipi/cache/dep_name/dep_version/
        let symlink_target = package_node_modules_dir.join(dep_name); // proyect/node_modules/.qipi/package_name@version/node_modules/dep_name

        if !Path::new(&cache_dep_path).exists() {
            download_tarball(
                get_tarball(
                    get_package(Package {
                        name: dep_name.to_string(),
                        version: dep_version.to_string(),
                        author: "".to_string(),
                        registry: "npm".to_string(),
                    })
                    .await
                    .unwrap()
                    .dist
                    .tarball,
                )
                .await
                .unwrap(),
                &get_cache_path(),
                dep_name,
                dep_version,
            )
            .unwrap();
        }

        if !Path::new(&symlink_target).exists() {
            symlink_auto(&cache_dep_path, &symlink_target).unwrap(); // proyect/node_modules/.qipi/package_name@version/node_modules/dep_name -> home/.qipi/cache/dep_name/dep_version/
        }

        let symlink_target_in_project = node_modules_dir.join(dep_name); // proyect/node_modules/dep_name

        if !Path::new(&symlink_target_in_project).exists() {
            symlink_auto(&symlink_target, &symlink_target_in_project).unwrap(); // proyect/node_modules/dep_name -> proyect/node_modules/.qipi/package_name@version/node_modules/dep_name
        }

        already_resolved.insert(dep_name.clone(), dep_version.clone());

        if let Some(dep_deps) = get_package(Package {
            name: dep_name.to_string(),
            version: dep_version.to_string(),
            author: "".to_string(),
            registry: "npm".to_string(),
        })
        .await
        .unwrap()
        .dependencies
        {
            if !dep_deps.is_empty() {
                let dep_list: Vec<(String, String)> = dep_deps.into_iter().collect();
                Box::pin(link_dependency(
                    dep_name,
                    dep_version,
                    &dep_list,
                    already_resolved,
                ))
                .await;
            }
        }
    }
}

fn parse_version(version_str: &str) -> Result<String, String> {
    let re = Regex::new(r"^(>=|<=|>|<|=|~|\^)?\s*([0-9]+(?:\.[0-9]+)*(?:\.[0-9]+)?)").unwrap();

    if let Some(caps) = re.captures(version_str) {
        let version = caps.get(2).map_or("", |m| m.as_str());

        Ok(version.to_string())
    } else {
        Err(format!("Invalid version range format: {}", version_str))
    }
}

fn get_package_cache_path(name: &str, version: &str) -> String {
    let name = name.replace("/", "+").to_string();

    let cache_path = get_cache_path();
    format!("{cache_path}{MAIN_SEPARATOR_STR}{name}{MAIN_SEPARATOR_STR}{version}") // home/.qipi/cache/name/version
}
