use std::collections::HashMap;

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

use crate::{
    config::get_cache_path,
    core::{
        client::{http::get_tarball, response::{Package, Registry}},
        package::{
            cache::{
                link::{link_dependency, link_package},
                tarball::{download_tarball, has_tarball_in_cache},
            },
            lockfile::{load_lockfile, update_lockfile, Lockfile, PackageInfo},
        },
    },
};

use super::r#struct::{Commands, QipiCLI};

pub async fn init() -> Result<()> {
    let cli = QipiCLI::parse();

    match cli.cmds {
        Some(Commands::Add { packages, registry }) => {
            for package in packages {
                let mut package_parsed =
                    parse_package_entry(&package.as_str()).expect("Invalid package format");

                package_parsed.registry = registry.as_ref().unwrap_or(&Registry::NPM).clone();

                let package_obtained = package_parsed
                    .get_package()
                    .await
                    .context("Could not get package")?;
                println!("{:#?}", &package_obtained);

                if !has_tarball_in_cache(
                    &get_cache_path(),
                    &package_obtained.name,
                    &package_obtained.version,
                ) {
                    download_tarball(
                        get_tarball(package_obtained.dist.tarball.clone())
                            .await
                            .context("Could not get tarball")?,
                        &get_cache_path(),
                        &package_obtained.name,
                        &package_obtained.version,
                    )
                    .context("Could not download tarball")?;
                }

                link_package(&package_obtained.name, &package_obtained.version)
                    .context("Could not link package")?;

                let lockfile = load_lockfile(".").unwrap_or(Lockfile::new());

                if lockfile.dependencies.contains_key(&package_obtained.name) {
                    return Ok(());
                }

                let mut deps: Vec<String> = vec![];
                let mut dependencies_map: Vec<(String, String)> = vec![];
                let _ = &package_obtained.dependencies.map(|dependencies| {
                    for (dependency_name, dependency_version) in dependencies {
                        deps.push(dependency_name.clone());
                        dependencies_map
                            .push((dependency_name.clone(), dependency_version.clone()));
                    }
                });

                let mut already_resolved: HashMap<String, String> = HashMap::new();

                link_dependency(
                    &package_obtained.name,
                    &package_obtained.version,
                    &dependencies_map,
                    &mut already_resolved,
                )
                .await
                .context("Could not link dependencies")?;

                let package_info = PackageInfo {
                    version: package_obtained.version,
                    resolved: package_obtained.dist.tarball,
                    integrity: package_obtained.dist.integrity,
                    dependencies: deps,
                };

                update_lockfile(".", &package_obtained.name, package_info)
                    .expect("Could not update lockfile");
            }

            Ok(())
        }

        Some(Commands::Remove { packages: _ }) => Ok(()),

        Some(Commands::Install) => Ok(()),

        None => Ok(()),
    }
}

fn parse_package_entry(package: &str) -> Result<Package> {
    static PACKAGE_REGEX: &str = r"^(?:(?P<author>@[^/]+)/)?(?P<name>[^@]+)(?:@(?P<version>.+))?$";

    let re: Regex = Regex::new(PACKAGE_REGEX).context("Invalid package regex")?;

    if let Some(caps) = re.captures(package) {
        let author = caps
            .name("author")
            .map_or_else(String::new, |m| m.as_str().to_string());
        let name = caps
            .name("name")
            .map_or_else(String::new, |m| m.as_str().to_string());
        let version = caps
            .name("version")
            .map_or_else(|| "latest".to_string(), |m| m.as_str().to_string());

        Ok(Package {
            author,
            name,
            version,
            ..Default::default()
        })
    } else {
        Err(anyhow::anyhow!("Invalid package format: {}", package))
    }
}
