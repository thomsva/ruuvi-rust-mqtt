use bluer::AdapterEvent;
use futures_util::stream::StreamExt;

use std::collections::HashSet;

use tokio::time::{self, Duration};

mod decode_ruuvi; // declares the module
use decode_ruuvi::decode_ruuvi_raw5; // imports the function

mod config;
use config::load_config;

mod mqtt;
use mqtt::MqttHandler;

mod startup_info;
use startup_info::print_startup_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    // println!("{:#?}", config);

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

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

    let mut known_sensors: HashSet<String> = HashSet::new();

    loop {
        let mut events = adapter.discover_devices().await?;

        while let Ok(Some(AdapterEvent::DeviceAdded(addr))) =
            time::timeout(Duration::from_secs(3), events.next()).await
        {
            let mac = addr.to_string();

            // Device object based on addr
            let device = match adapter.device(addr) {
                Ok(d) => d,
                Err(_) => continue,
            };

            // Extract manufacturer data
            let manuf = match device.manufacturer_data().await {
                Ok(Some(m)) => m,
                _ => continue,
            };

            let data = match manuf.get(&1177) {
                Some(d) => d,
                None => continue,
            };

            // Determine if sensor is allowed by blacklist and whitelist rules
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
                continue; // Skip further processing
            }

            // Check if new sensor
            if !known_sensors.contains(&mac) {
                println!("New Ruuvi sensor detected and saved: {}", mac);
                known_sensors.insert(mac.clone());
                if config.publish.discovery {
                    mqtt.send_discovery(&mac).await.ok();
                }
            }

            if config.publish.raw_data {
                if let Err(e) = mqtt.publish_raw(&mac, &data).await {
                    eprintln!("❌ Failed to publish raw data for {}: {}", mac, e);
                }
            }

            match decode_ruuvi_raw5(&data) {
                Some((t, h, p)) => {
                    // Always print if debug_print_measurements is enabled
                    if config.sensors.debug_print {
                        println!("{} → {:.2}°C  {:.1}%  {:.1}hPa", mac, t, h, p);
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

        // Wait 5 seconds before next discovery cycle
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
