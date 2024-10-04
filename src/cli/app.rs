use clap::Parser;
use regex::Regex;

use crate::{
    config::get_cache_path,
    core::{
        client::{
            http::{get_package, get_tarball},
            response::Package,
        },
        package::{
            cache::{
                link::link_package,
                tarball::{download_tarball, has_tarball_in_cache},
            },
            lockfile::{load_lockfile, update_lockfile, Lockfile, PackageInfo},
        },
    },
};

use super::r#struct::{Commands, QipiCLI};

pub async fn init() {
    let cli = QipiCLI::parse();

    match cli.cmds {
        Some(Commands::Add { packages, registry }) => {
            for package in packages {
                let mut package_parsed = parse_package_entry(&package.as_str()).unwrap();
                let registry = registry.clone().unwrap_or("npm".to_string());

                package_parsed.registry = registry;

                let package_obtained = get_package(package_parsed).await.unwrap();
                println!("{:#?}", &package_obtained);

                if !has_tarball_in_cache(
                    &get_cache_path(),
                    &package_obtained.name,
                    &package_obtained.version,
                ) {
                    download_tarball(
                        get_tarball(package_obtained.dist.tarball.clone())
                            .await
                            .unwrap(),
                        &get_cache_path(),
                        &package_obtained.name,
                        &package_obtained.version,
                    )
                    .unwrap();
                }

                link_package(&package_obtained.name, &package_obtained.version);

                let lockfile = load_lockfile(".").unwrap_or(Lockfile::new());

                if lockfile.dependencies.contains_key(&package_obtained.name) {
                    return;
                }

                let mut deps: Vec<String> = vec![];
                let _ = &package_obtained.dependencies.map(|dependencies| {
                    for (dependency_name, _) in dependencies {
                        deps.push(dependency_name);
                    }
                });

                let package_info = PackageInfo {
                    version: package_obtained.version,
                    resolved: package_obtained.dist.tarball,
                    integrity: package_obtained.dist.integrity,
                    dependencies: deps,
                };

                update_lockfile(".", &package_obtained.name, package_info).unwrap();
            }
        }

        Some(Commands::Remove { packages }) => (),

        Some(Commands::Install) => (),

        None => (),
    }
}

fn parse_package_entry(package: &str) -> Result<Package, String> {
    let re = Regex::new(r"^(?:(?P<author>@[^/]+)/)?(?P<name>[^@]+)(?:@(?P<version>.+))?$").unwrap();

    if let Some(caps) = re.captures(package) {
        let author = caps
            .name("author")
            .map(|m| m.as_str().to_string())
            .unwrap_or("".to_string());
        let name = caps["name"].to_string();
        let version = caps
            .name("version")
            .map(|m| m.as_str().to_string())
            .unwrap_or("latest".to_string());

        Ok(Package {
            author,
            name,
            version,
            registry: "npm".to_string(),
        })
    } else {
        Err(format!("Invalid package format: {}", package))
    }
}
