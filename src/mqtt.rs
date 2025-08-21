use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use std::time::Duration;
use tokio::task;

pub struct MqttHandler {
    client: AsyncClient,
}

impl MqttHandler {
    /// Initialize MQTT connection
    pub async fn new(client_id: &str, host: &str, port: u16) -> Self {
        let mut mqttoptions = MqttOptions::new(client_id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

        // Run the event loop in the background
        task::spawn(async move {
            loop {
                if let Err(e) = eventloop.poll().await {
                    eprintln!("MQTT error: {:?}", e);
                }
            }
        });

        Self { client }
    }

    /// Publish sensor data
    pub async fn publish_sensor(
        &self,
        mac: &str,
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
}
