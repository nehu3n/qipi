use anyhow::{Context, Result};
use reqwest::Client;

use super::response::{NPMPackage, Package, Registry};

impl Package {
    pub async fn get_package(self) -> Result<NPMPackage> {
        let name = if !self.author.is_empty() {
            format!("{}/{}", self.author, self.name)
        } else {
            self.name
        };

        match &self.registry {
            Registry::NPM => {
                let registry_url = format!("https://registry.npmjs.org/{}/{}", name, self.version);

                let client = Client::new();
                let response = client
                    .get(&registry_url)
                    .send()
                    .await
                    .context("Error fetching package from registry")?;
                let package = response
                    .json::<NPMPackage>()
                    .await
                    .context("Error parsing package")?;

                Ok(package)
            }
            Registry::JSR => {
                todo!("Fetching packages from the JSR registry has not yet been implemented.")
            }
        }
    }
}

pub async fn get_tarball(url: String) -> Result<Vec<u8>> {
    let response = reqwest::get(url).await.context("Error fetching tarball")?;
    let tarball = response
        .bytes()
        .await
        .context("Error streaming tarball")?
        .to_vec();

    Ok(tarball)
}
