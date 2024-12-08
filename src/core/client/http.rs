use super::response::{NPMPackage, Package, Registry};

pub async fn get_package(package: Package) -> Result<NPMPackage, String> {
    let name = if !package.author.is_empty() {
        &format!("{}/{}", package.author, package.name)
    } else {
        &package.name
    };

    match &package.registry {
        Registry::NPM => {
            let registry_url = format!("https://registry.npmjs.org/{}/{}", name, package.version);

            let response = reqwest::get(registry_url).await.unwrap();
            let package = response.json::<NPMPackage>().await.unwrap();

            return Ok(package);
        }
        Registry::JSR => {
            todo!("Fetching packages from the JSR registry has not yet been implemented.")
        }
    }
}

pub async fn get_tarball(url: String) -> Result<Vec<u8>, String> {
    let response = reqwest::get(url).await.unwrap();
    let tarball = response.bytes().await.unwrap().to_vec();

    Ok(tarball)
}
