use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use std::time::Duration;
use tokio::sync::watch;
use tokio::task;

pub struct MqttHandler {
    client: AsyncClient,
    connection_status: watch::Receiver<bool>, // true = connected, false = disconnected
}

impl MqttHandler {
    /// Initialize MQTT connection
    pub async fn new(client_id: &str, host: &str, port: u16) -> Self {
        let mut mqttoptions = MqttOptions::new(client_id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Channel to broadcast connection status
        let (status_tx, status_rx) = watch::channel(true);

        // Run the event loop in the background
        task::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(_) => {
                        let _ = status_tx.send(true);
                    }
                    Err(e) => {
                        eprintln!("❌ MQTT error: {:?}", e);
                        let _ = status_tx.send(false);
                        // small delay to avoid tight loop on connection loss
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });

        Self {
            client,
            connection_status: status_rx,
        }
    }

    /// Publish sensor data and print reading + MQTT status
    pub async fn publish_sensor(&self, mac: &str, t: f32, h: f32, p: f32) {
        let topic = format!("ruuvi/{}/raw5", mac);
        let payload = format!("{{\"temp\":{:.2},\"hum\":{:.1},\"pres\":{:.1}}}", t, h, p);

        let status = if *self.connection_status.borrow() {
            match self
                .client
                .publish(topic, QoS::AtLeastOnce, false, payload)
                .await
            {
                Ok(_) => "[MQTT: success]",
                Err(_) => "[MQTT: error]",
            }
        } else {
            "[MQTT: error]"
        };

        println!("{} → {:.2}°C  {:.1}%  {:.1}hPa {}", mac, t, h, p, status);
    }
}
