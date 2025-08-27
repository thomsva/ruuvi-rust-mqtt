use bluer::{Adapter, AdapterEvent, Address, DiscoveryFilter, DiscoveryTransport};
use futures_util::stream::StreamExt;
use tokio::time::{Duration, sleep};

use std::collections::{HashMap, HashSet};

mod decode_ruuvi; // declares the module
use decode_ruuvi::decode_ruuvi_raw5; // imports the function

mod config;
use config::load_config;

mod mqtt;
use mqtt::MqttHandler;

mod startup_info;
use startup_info::print_startup_info;

use crate::startup_info::print_version_info;
const RUUVI_COMPANY_ID: u16 = 0x0499;

async fn extract_ruuvi_payload(
    adapter: &Adapter,
    mac: &Address,
    config: &config::Config,
) -> Option<Vec<u8>> {
    let device = adapter.device(*mac).ok()?; // return None if failed
    let blocked_by_whitelist =
        config.sensors.use_whitelist && !config.sensors.whitelist.contains(&mac);
    let blocked_by_blacklist =
        config.sensors.use_blacklist && config.sensors.blacklist.contains(&mac);
    let allowed = !blocked_by_whitelist && !blocked_by_blacklist;

    if !allowed && config.sensors.debug_print {
        if blocked_by_whitelist {
            println!("{} → blocked by whitelist", mac);
        } else if blocked_by_blacklist {
            println!("{} → blocked by blacklist", mac);
        }
    }

    if !allowed {
        return None;
    }

    let data = device.manufacturer_data().await.ok()??; // double ? for Option inside Result
    data.get(&RUUVI_COMPANY_ID).cloned() // return Some(payload) or None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_version_info(env!("CARGO_PKG_VERSION"));

    let config = load_config()?;

    let session = bluer::Session::new().await?;

    let adapter = loop {
        match session.default_adapter().await {
            Ok(a) => break a,
            Err(e) => {
                eprintln!(
                    "❌ No default Bluetooth adapter found: {}. Retrying in 10s…",
                    e
                );
                sleep(Duration::from_secs(10)).await;
            }
        }
    };

    adapter.set_powered(true).await?;

    let filter = DiscoveryFilter {
        transport: DiscoveryTransport::Le,
        discoverable: false,
        ..Default::default()
    };

    adapter.set_discovery_filter(filter).await?;

    print_startup_info(&config, &adapter).await;

    // Setup MQTT
    let mqtt = MqttHandler::new(
        "ruuvi-client",
        &config.mqtt.host,
        config.mqtt.port,
        config.mqtt.username.as_deref(),
        config.mqtt.password.as_deref(),
    )
    .await;

    let mut events = adapter.discover_devices_with_changes().await?;
    let mut previous_payload: HashMap<Address, Vec<u8>> = HashMap::new();
    let mut known_sensors: HashSet<Address> = HashSet::new();

    while let Some(event) = events.next().await {
        match event {
            AdapterEvent::DeviceAdded(mac) => {
                // Get Ruuvi payload if possible
                let Some(payload) = extract_ruuvi_payload(&adapter, &mac, &config).await else {
                    continue;
                };

                // Remove duplicates
                if previous_payload.get(&mac) == Some(&payload) {
                    continue;
                }
                previous_payload.insert(mac.clone(), payload.clone());

                // Publish discovery if new sensor
                if !known_sensors.contains(&mac) {
                    println!("New Ruuvi sensor detected and saved: {}", mac);
                    known_sensors.insert(mac.clone());
                    if config.publish.discovery {
                        mqtt.send_discovery(&mac.to_string()).await.ok();
                    }
                }

                // Publish raw data
                if config.publish.raw_data {
                    if let Err(e) = mqtt.publish_raw(&mac.to_string(), &payload).await {
                        eprintln!("❌ Failed to publish raw data for {}: {}", mac, e);
                    }
                }

                // Publish decoded data
                match decode_ruuvi_raw5(&payload) {
                    Some((t, h, p, b)) => {
                        // Always print if debug_print_measurements is enabled
                        if config.sensors.debug_print {
                            println!("{} → {:.2}°C  {:.1}%  {:.1}hPa  {:.3}V", mac, t, h, p, b);
                        }

                        // Publish if enabled
                        if config.publish.decoded_data {
                            if let Err(e) = mqtt.publish_decoded(&mac, t, h, p).await {
                                eprintln!("❌ Failed to publish decoded data for {}: {}", mac, e);
                            }
                        }
                    }
                    None => {
                        if config.sensors.debug_print {
                            eprintln!("⚠️ Failed to decode Ruuvi data from {}", mac);
                        }
                    }
                }
            }

            AdapterEvent::DeviceRemoved(_mac) => {}
            AdapterEvent::PropertyChanged(_p) => {}
        }
    }
    Ok(())
}
