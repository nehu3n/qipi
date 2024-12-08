use std::{
    collections::HashMap,
    fs::create_dir_all,
    path::{Path, PathBuf, MAIN_SEPARATOR_STR},
};

use anyhow::{Context, Result};
use regex::Regex;
use symlink::symlink_auto;

use crate::{
    config::get_cache_path,
    core::client::{http::get_tarball, response::Package},
};

use super::tarball::download_tarball;

fn create_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        create_dir_all(path).context("Failed to create directory")?;
    }
    Ok(())
}

fn create_symlink(target: &Path, link: &Path) -> Result<()> {
    if !link.exists() {
        symlink_auto(target, link).context(format!(
            "Failed to create symlink from {} to {}",
            link.display(),
            target.display()
        ))?;
    }
    Ok(())
}

pub fn link_package(name: &str, version: &str) -> Result<()> {
    println!("linking {}@{}", name, version);

    let package_cache_path = get_package_cache_path(name, version); // home/.qipi/cache/name/version/
    let project_root = Path::new("."); // proyect/
    let node_modules_dir = project_root.join("node_modules"); // proyect/node_modules
    let node_modules_qipi_dir = node_modules_dir.join(".qipi"); // proyect/node_modules/.qipi
    let package_versioned_dir = node_modules_qipi_dir.join(format!("{}@{}", name, version)); // proyect/node_modules/.qipi/name@version
    let package_node_modules_dir = package_versioned_dir.join("node_modules"); // proyect/node_modules/.qipi/name@version/node_modules

    create_directory(&package_node_modules_dir)?; // proyect/node_modules/.qipi/name@version/node_modules

    let cache_package_path = PathBuf::from(&package_cache_path); // home/.qipi/cache/name/version/
    let symlink_target = package_node_modules_dir.join(name); // proyect/node_modules/.qipi/name@version/node_modules/name

    create_symlink(&cache_package_path, &symlink_target)?; // proyect/node_modules/.qipi/name@version/node_modules/name -> home/.qipi/cache/name/version

    let symlink_target_in_project = node_modules_dir.join(name); // proyect/node_modules/name
    create_symlink(&symlink_target, &symlink_target_in_project)?; // proyect/node_modules/name -> proyect/node_modules/.qipi/name@version/node_modules/name

    Ok(())
}

async fn download_and_symlink_dependency(
    package: &Package,
    dep_name: &str,
    dep_version: &str,
    package_node_modules_dir: &Path,
) -> Result<()> {
    let cache_dep_path = get_package_cache_path(dep_name, dep_version);

    if !Path::new(&cache_dep_path).exists() {
        download_tarball(
            get_tarball(package.clone().get_package().await.unwrap().dist.tarball)
                .await
                .unwrap(),
            &get_cache_path(),
            dep_name,
            dep_version,
        )?;
    }

    let symlink_target = package_node_modules_dir.join(dep_name);
    create_symlink(&PathBuf::from(&cache_dep_path), &symlink_target)?;

    let node_modules_dir = Path::new(".").join("node_modules");
    let symlink_target_in_project = node_modules_dir.join(dep_name);
    create_symlink(&symlink_target, &symlink_target_in_project)?;

    Ok(())
}

pub async fn link_dependency(
    package_name: &str,
    version: &str,
    dependencies: &[(String, String)],
    already_resolved: &mut HashMap<String, String>,
) -> Result<()> {
    let project_root = Path::new("."); // proyect/
    let node_modules_dir = project_root.join("node_modules"); // proyect/node_modules/
    let node_modules_qipi_dir = node_modules_dir.join(".qipi"); // proyect/node_modules/.qipi/
    let package_versioned_dir = node_modules_qipi_dir.join(format!("{}@{}", package_name, version)); // proyect/node_modules/.qipi/package_name@version/
    let package_node_modules_dir = package_versioned_dir.join("node_modules"); // proyect/node_modules/.qipi/package_name@version/node_modules/

    create_directory(&package_node_modules_dir)?; // proyect/node_modules/.qipi/package_name@version/node_modules/

    for (dep_name, dep_version) in dependencies {
        let dep_version = &parse_version(&dep_version)?;

        let package = Package {
            name: dep_name.to_string(),
            version: dep_version.to_string(),
            ..Default::default()
        };

        if let Some(existing_version) = already_resolved.get(dep_name) {
            if existing_version != dep_version {
                let unique_symlink_target =
                    node_modules_qipi_dir.join(format!("{}@{}", dep_name, dep_version));

                if !Path::new(&unique_symlink_target).exists() {
                    download_and_symlink_dependency(
                        &package,
                        dep_name,
                        dep_version,
                        &package_node_modules_dir,
                    )
                    .await?;
                }
                continue;
            }
        }

        download_and_symlink_dependency(&package, dep_name, dep_version, &package_node_modules_dir)
            .await?;

        already_resolved.insert(dep_name.clone(), dep_version.clone());

        if let Some(dep_deps) = package.get_package().await.unwrap().dependencies {
            if !dep_deps.is_empty() {
                let dep_list: Vec<(String, String)> = dep_deps.into_iter().collect();
                Box::pin(link_dependency(
                    dep_name,
                    dep_version,
                    &dep_list,
                    already_resolved,
                ))
                .await
                .context("Failed to link dependency")?;
            }
        }
    }

    Ok(())
}

fn parse_version(version_str: &str) -> Result<String> {
    let re = Regex::new(r"^(>=|<=|>|<|=|~|\^)?\s*([0-9]+(?:\.[0-9]+)*(?:\.[0-9]+)?)")
        .context("Invalid version range format")?;

    match re.captures(version_str) {
        Some(caps) => {
            let version = caps.get(2).map_or("", |m| m.as_str());

            Ok(version.to_string())
        }
        None => Err(anyhow::anyhow!(
            "Invalid version range format: {}",
            version_str
        )),
    }
}

fn get_package_cache_path(name: &str, version: &str) -> String {
    let name = name.replace("/", "+").to_string();

    let cache_path = get_cache_path();
    format!("{cache_path}{MAIN_SEPARATOR_STR}{name}{MAIN_SEPARATOR_STR}{version}")
    // home/.qipi/cache/name/version
}
