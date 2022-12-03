/* Annotation trait */
use serde_derive::Deserialize;

/* Root Toml Config object. */
#[derive(Deserialize)]
pub struct Config {
    pub metadata: Metadata,
    pub spec: Spec,
}

/* Metadata Toml Config object. */
#[derive(Deserialize)]
pub struct Metadata {
    pub description: Description,
}

/* Spec Toml Config object. */
#[derive(Deserialize)]
pub struct Spec {
    pub server: Server,
}

/* Metadata.Description Toml Config object. */
#[derive(Deserialize)]
pub struct Description {
    pub short: String,
    pub long: String,
}

/* Spec.Server Toml Config object. */
#[derive(Deserialize)]
pub struct Server {
    pub port: u16,
}

/* Helper function to read Toml config file. */
pub async fn read_toml_config(file: &String) -> Result<Config, std::io::Error> {
    println!("[DEBUG] Attempting to read Toml config file '{}'.", file);
    let toml_str = std::fs::read_to_string(file)?;
    let config: Config = toml::from_str(&toml_str)?;
    let result = Ok(config);
    println!("[DEBUG] Successfully read Toml config file '{}'.", file);
    return result;
}
