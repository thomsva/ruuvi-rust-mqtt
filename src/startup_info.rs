// src/print_startup_info.rs

use crate::config::Config;
use bluer::Adapter;

pub async fn print_startup_info(config: &Config, adapter: &Adapter) {
    // Print a clean header with optional version
    println!("Ruuvi Rust MQTT v0.1.0");
    println!("----------------------");

    // MQTT info
    let mqtt_username = config.mqtt.username.as_deref().unwrap_or("none");
    println!(
        "MQTT: {}:{} (username: {})",
        config.mqtt.host, config.mqtt.port, mqtt_username
    );

    // Adapter info
    let adapter_name = adapter.name();
    let powered = adapter.is_powered().await.unwrap_or(false);
    println!("Adapter: {} powered {}", adapter_name, powered);

    // Whitelist / blacklist
    println!(
        "Whitelist: {}{}",
        if config.sensors.use_whitelist {
            "enabled"
        } else {
            "disabled"
        },
        if !config.sensors.whitelist.is_empty() {
            format!(" ({})", config.sensors.whitelist.len())
        } else {
            "".to_string()
        }
    );
    println!(
        "Blacklist: {}{}",
        if config.sensors.use_blacklist {
            "enabled"
        } else {
            "disabled"
        },
        if !config.sensors.blacklist.is_empty() {
            format!(" ({})", config.sensors.blacklist.len())
        } else {
            "".to_string()
        }
    );

    // Debug print
    println!(
        "Debug print: {}",
        if config.sensors.debug_print {
            "enabled"
        } else {
            "disabled"
        }
    );

    // Publish settings
    println!(
        "Discovery publishing: {}",
        if config.publish.discovery {
            "enabled"
        } else {
            "disabled"
        }
    );
    println!(
        "Decoded data publishing: {}",
        if config.publish.decoded_data {
            "enabled"
        } else {
            "disabled"
        }
    );
    println!(
        "Raw data publishing: {}",
        if config.publish.raw_data {
            "enabled"
        } else {
            "disabled"
        }
    );

    println!("----------------------\n");
}
