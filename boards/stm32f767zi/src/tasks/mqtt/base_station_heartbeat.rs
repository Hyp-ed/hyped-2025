use super::send::MQTT_SEND;
use core::str::FromStr;
use defmt::debug;
use embassy_time::{Duration, Timer};
use heapless::String;
use hyped_core::{mqtt::MqttMessage, mqtt_topics::MqttTopic};
use {defmt_rtt as _, panic_probe as _};

/// Sends a heartbeat message to the MQTT broker every second.
#[embassy_executor::task]
pub async fn base_station_heartbeat() {
    loop {
        MQTT_SEND
            .send(MqttMessage::new(
                MqttTopic::Heartbeat,
                String::<512>::from_str("").unwrap(),
            ))
            .await;
        debug!("Sent heartbeat message");
        Timer::after(Duration::from_millis(100)).await;
    }
}
