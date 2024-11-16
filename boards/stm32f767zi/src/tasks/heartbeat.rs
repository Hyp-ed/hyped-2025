use super::mqtt::SEND_CHANNEL;
use core::str::FromStr;
use defmt::debug;
use embassy_time::{Duration, Timer};
use heapless::String;
use hyped_core::{mqtt::MqttMessage, mqtt_topics::MqttTopics};
use {defmt_rtt as _, panic_probe as _};

/// Sends a heartbeat message to the MQTT broker every second
#[embassy_executor::task]
pub async fn heartbeat() {
    loop {
        debug!("Sending heartbeat...");
        SEND_CHANNEL
            .send(MqttMessage {
                topic: MqttTopics::to_string(&MqttTopics::Heartbeat),
                payload: String::<512>::from_str("").unwrap(),
            })
            .await;
        Timer::after(Duration::from_secs(1)).await;
    }
}
