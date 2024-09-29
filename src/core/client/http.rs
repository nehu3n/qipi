use super::response::NPMPackage;

pub async fn get_package(
    name: &str,
    version: Option<&str>,
    registry: &str,
) -> Result<NPMPackage, String> {
    if registry == "npm" {
        let registry_url = format!(
            "https://registry.npmjs.org/{}/{}",
            name,
            version.unwrap_or("latest")
        );

        let response = reqwest::get(registry_url).await.unwrap();
        let package = response.json::<NPMPackage>().await.unwrap();

        Ok(package)
    } else if registry == "jsr" {
        todo!("Fetching packages from the JSR registry has not yet been implemented.")
    } else {
        return Err("The registry passed as a parameter must be “npm” or “jsr”.".to_string());
    }
}