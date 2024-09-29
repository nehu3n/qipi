use super::response::{NPMPackage, Package};

pub async fn get_package(package: Package) -> Result<NPMPackage, String> {
    let name = if !package.author.is_empty() {
        &format!("{}/{}", package.author, package.name)
    } else {
        &package.name
    };

    if &package.registry == "npm" {
        let registry_url = format!("https://registry.npmjs.org/{}/{}", name, package.version);

        let response = reqwest::get(registry_url).await.unwrap();
        let package = response.json::<NPMPackage>().await.unwrap();

        Ok(package)
    } else if &package.registry == "jsr" {
        todo!("Fetching packages from the JSR registry has not yet been implemented.")
    } else {
        return Err("The registry passed as a parameter must be “npm” or “jsr”.".to_string());
    }
}
