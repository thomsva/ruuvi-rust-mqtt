use crate::config::Config;
use bluer::Adapter;
use std::collections::HashSet;

/// Essential BLE UUIDs for sensors
const REQUIRED_UUIDS: &[(&str, &str)] = &[
    ("1800", "Generic Access (required)"),
    ("1801", "Generic Attribute / GATT (required)"),
    ("180A", "Device Information (optional)"),
    ("180F", "Battery Service (optional)"),
    ("181A", "Environmental Sensing (optional)"),
];

const SEPARATOR_LENGTH: usize = 50;

/// Checks if required UUIDs are supported and prints a compact summary
async fn print_adapter_ble_services(adapter: &Adapter) {
    match adapter.uuids().await {
        Ok(Some(uuids)) => {
            // Extract 16-bit UUID prefixes for easy matching
            let uuid_shorts: HashSet<String> = uuids
                .iter()
                .filter_map(|u| {
                    let s = u.to_string();
                    if s.len() >= 8 {
                        Some(s[4..8].to_uppercase())
                    } else {
                        None
                    }
                })
                .collect();

            println!("  BLE Services:");
            let mut all_required_present = true;
            for (uuid, desc) in REQUIRED_UUIDS {
                let supported = uuid_shorts.contains(&uuid.to_string());
                if desc.contains("(required)") && !supported {
                    all_required_present = false;
                }
                println!(
                    "    {:<25} {}",
                    desc,
                    if supported { "supported" } else { "missing" }
                );
            }

            if all_required_present {
                println!("  All required BLE services are supported ✅");
            } else {
                println!(
                    "  ⚠️ Some required BLE services are missing. BLE devices may not be reachable."
                );
            }
        }
        Ok(None) => println!("  BLE Services: None reported"),
        Err(e) => println!("  BLE Services: <error: {}>", e),
    }
}

pub fn print_version_info(version: &str) {
    println!("{}", "=".repeat(SEPARATOR_LENGTH));
    println!("Ruuvi Rust MQTT v{}", version);
    println!("{}", "=".repeat(SEPARATOR_LENGTH));
}

pub async fn print_startup_info(config: &Config) {
    // MQTT info
    let mqtt_username = match config.mqtt.username.as_deref() {
        Some("") | None => "<none>",
        Some(name) => name,
    };

    let mqtt_password = match config.mqtt.password.as_deref() {
        Some("") | None => "<none>",
        Some(_) => "****", // hide actual password
    };

    println!(
        "MQTT: {}:{} (username: {}, password: {})",
        config.mqtt.host, config.mqtt.port, mqtt_username, mqtt_password
    );

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
            String::new()
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
            String::new()
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

    println!("{}", "-".repeat(SEPARATOR_LENGTH));
}

pub async fn print_adapter_info(adapter: &Adapter) {
    // Bluetooth session & adapter debug info
    println!("{}", "-".repeat(SEPARATOR_LENGTH));
    println!("Bluetooth Adapter Info:");
    println!("  Name:           {}", adapter.name());

    // Address (async)
    match adapter.address().await {
        Ok(addr) => println!("  Address:        {}", addr),
        Err(e) => println!("  Address:        <error: {}>", e),
    }

    // Powered, Discoverable, Pairable
    let powered = adapter.is_powered().await.unwrap_or(false);
    println!("  Powered:        {}", powered);
    println!(
        "  Discoverable:   {}",
        adapter.is_discoverable().await.unwrap_or(false)
    );
    println!(
        "  Pairable:       {}",
        adapter.is_pairable().await.unwrap_or(false)
    );

    // Discoverable & Pairable timeouts
    println!(
        "  Discoverable timeout: {:?}",
        adapter.discoverable_timeout().await.unwrap_or_default()
    );
    println!(
        "  Pairable timeout:     {:?}",
        adapter.pairable_timeout().await.unwrap_or_default()
    );

    // Print compact BLE service info
    print_adapter_ble_services(adapter).await;

    println!("{}", "-".repeat(SEPARATOR_LENGTH));
}
