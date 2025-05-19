use super::send::MQTT_SEND;
use core::str::FromStr;
use defmt::debug;
use defmt_rtt as _;
use embassy_time::{Duration, Timer};
use heapless::String;
use hyped_core::{config::HEARTBEAT_CONFIG, mqtt::MqttMessage, mqtt_topics::MqttTopic};
use panic_probe as _;

/// Sends a heartbeat message to the MQTT broker a
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

        Timer::after(Duration::from_hz(
            HEARTBEAT_CONFIG.base_station.frequency as u64,
        ))
        .await;
    }
}
