use serde::Deserialize;
use std::{collections::HashSet, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub sensors: SensorConfig,
}

#[derive(Debug, Deserialize)]
pub struct MqttConfig {
    pub broker: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SensorConfig {
    pub enable_discovery: bool,
    pub decode_data: bool,
    pub whitelist: Option<HashSet<String>>, // changed from Vec<String>
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
