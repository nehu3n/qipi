use clap::Parser;
use regex::Regex;

use crate::core::client::http::get_package;

use super::r#struct::{Commands, QipiCLI};

pub async fn init() {
    let cli = QipiCLI::parse();

    match cli.cmds {
        Some(Commands::Add { packages }) => {
            for package in packages {
                let package_parsed = parse_package_entry(&package.as_str());
                let registry = "npm"; // TODO: Possibility to indicate registry by means of a flag "--registry <npm|jsr>".
                match package_parsed {
                    Ok(pkg) => {
                        let name = if !pkg.author.is_empty() {
                            &format!("{}/{}", pkg.author, pkg.name)
                        } else {
                            &pkg.name
                        };

                        let package_obtained = get_package(name, Some(&pkg.version), registry)
                            .await
                            .unwrap();

                        println!("{:#?}", package_obtained)
                    }
                    Err(e) => panic!("{}", e),
                }
            }
        }

        Some(Commands::Remove { packages }) => (),

        Some(Commands::Install) => (),

        None => (),
    }
}

struct Package {
    pub author: String,
    pub name: String,
    pub version: String,
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
        })
    } else {
        Err(format!("Invalid package format: {}", package))
    }
}
