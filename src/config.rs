use bluer::Address;
use serde::Deserialize;
use std::{collections::HashSet, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub sensors: SensorConfig,
    pub publish: PublishConfig,
}

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SensorConfig {
    pub blacklist: HashSet<Address>,
    pub use_blacklist: bool,
    pub whitelist: HashSet<Address>,
    pub use_whitelist: bool,
    pub debug_print: bool,
}

#[derive(Debug, Deserialize)]
pub struct PublishConfig {
    pub discovery: bool,
    pub decoded_data: bool,
    pub raw_data: bool,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new("config.toml");
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
