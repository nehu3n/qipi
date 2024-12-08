use reqwest::Client;

use super::response::{NPMPackage, Package, Registry};

impl Package {
    pub async fn get_package(self) -> Result<NPMPackage, String> {
        let name = if !self.author.is_empty() {
            format!("{}/{}", self.author, self.name)
        } else {
            self.name
        };

        match &self.registry {
            Registry::NPM => {
                let registry_url = format!("https://registry.npmjs.org/{}/{}", name, self.version);

                let client = Client::new();
                let response = client.get(&registry_url).send().await.unwrap();
                let package = response.json::<NPMPackage>().await.unwrap();

                Ok(package)
            }
            Registry::JSR => {
                todo!("Fetching packages from the JSR registry has not yet been implemented.")
            }
        }
    }
}

pub async fn get_tarball(url: String) -> Result<Vec<u8>, String> {
    let response = reqwest::get(url).await.unwrap();
    let tarball = response.bytes().await.unwrap().to_vec();

    Ok(tarball)
}
