use bluer::AdapterEvent;
use futures_util::stream::StreamExt;

use std::collections::HashSet;

use tokio::time::{Duration, timeout};

mod decode_ruuvi; // declares the module
use decode_ruuvi::decode_ruuvi_raw5; // imports the function

mod config;
use config::load_config;

mod mqtt;
use mqtt::MqttHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    println!("{:#?}", config);

    // Setup MQTT
    let mqtt = MqttHandler::new("ruuvi-client", "localhost", 1883).await;

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    println!("Adapter powered on: {:?}", adapter.is_powered().await?);
    let mut known_sensors: HashSet<String> = HashSet::new();

    println!("Starting discovery...");

    loop {
        let mut events = adapter.discover_devices().await?;

        // Discovery every 5 seconds
        let _ = timeout(Duration::from_secs(5), async {
            while let Some(AdapterEvent::DeviceAdded(addr)) = events.next().await {
                // Check if Ruuvi sensor
                let mac = addr.to_string();
                if !mac.starts_with("F1:CC:CA") {
                    continue;
                }

                // Check if new sensor
                if !known_sensors.contains(&mac) {
                    println!("New Ruuvi sensor detected and saved: {}", mac);
                    known_sensors.insert(mac.clone());
                }

                // Device object based on addr
                let device = match adapter.device(addr) {
                    Ok(d) => d,
                    Err(_) => continue,
                };

                // Extract data
                let manuf = match device.manufacturer_data().await {
                    Ok(Some(m)) => m,
                    _ => continue,
                };

                // Match with Ruuvi manufacturer id present in Ruuvi RAVv5 format data
                let data = match manuf.get(&1177) {
                    Some(d) => d,
                    None => continue,
                };

                // Decode data and publish if successful
                if let Some((t, h, p)) = decode_ruuvi_raw5(data) {
                    //println!("---------{}  →  {:.2}°C  {:.1}%  {:.1}hPa", mac, t, h, p);

                    // call function to send data to mqtt
                    let _ = mqtt.publish_sensor(&mac, t, h, p).await;
                }
            }
        })
        .await;
    }
}
