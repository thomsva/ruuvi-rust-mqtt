use bluer::Address;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;
use std::time::Duration;
use tokio::task;

pub struct MqttHandler {
    client: AsyncClient,
}

impl MqttHandler {
    /// Create a new MQTT handler and start the background event loop
    pub async fn new(
        client_id: &str,
        host: &str,
        port: u16,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Self {
        let mut mqttoptions = MqttOptions::new(client_id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        if let Some(username) = username {
            let password = password.unwrap_or("");
            mqttoptions.set_credentials(username, password);
        }

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Background task to handle connection and errors
        task::spawn(async move {
            let mut connected = false;
            loop {
                match eventloop.poll().await {
                    Ok(_) => {
                        if !connected {
                            println!("✅ MQTT connected");
                            connected = true;
                        }
                    }
                    Err(e) => {
                        if connected {
                            eprintln!("❌ MQTT error: {:?}", e);
                            connected = false;
                        }
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });

        Self { client }
    }

    pub async fn publish_decoded(
        &self,
        mac: &Address,
        t: f32,
        h: f32,
        p: f32,
    ) -> Result<(), rumqttc::ClientError> {
        let topic = format!("ruuvi/{}/raw5", mac);
        let payload = format!("{{\"temp\":{:.2},\"hum\":{:.1},\"pres\":{:.1}}}", t, h, p);
        self.client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
    }

    pub async fn publish_raw(&self, mac: &str, raw: &[u8]) -> Result<(), rumqttc::ClientError> {
        let topic = format!("ruuvi/{}/raw", mac);
        let payload = hex::encode(raw);
        self.client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
    }

    pub async fn send_discovery(&self, mac: &str) -> Result<(), rumqttc::ClientError> {
        let base_topic = format!("homeassistant/sensor/ruuvi_{}/", mac.replace(":", "_"));
        let device_id = format!("ruuvi_{}", mac.replace(":", "_"));
        let device_name = format!("Ruuvi {}", mac);

        let sensors = vec![
            ("temperature", "Temperature", "°C", "{{ value_json.temp }}"),
            ("humidity", "Humidity", "%", "{{ value_json.hum }}"),
            ("pressure", "Pressure", "hPa", "{{ value_json.pres }}"),
        ];

        for (object_id, name, unit, value_template) in sensors {
            let topic = format!("{}{}/config", base_topic, object_id);
            let payload = json!({
                "name": name,
                "state_topic": format!("ruuvi/{}/raw5", mac),
                "unit_of_measurement": unit,
                "value_template": value_template,
                "unique_id": format!("{}_{}", device_id, object_id),
                "device": {
                    "identifiers": [device_id],
                    "name": device_name,
                    "manufacturer": "Ruuvi",
                    "model": "RuuviTag"
                }
            });
            self.client
                .publish(topic, QoS::AtLeastOnce, true, payload.to_string())
                .await?;
        }

        Ok(())
    }
}
