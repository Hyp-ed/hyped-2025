use crate::tasks::mqtt::send::MQTT_SEND;
use core::str::FromStr;
use defmt::{debug, error, info, warn};
use heapless::String;
use hyped_core::{log_types::LogLevel, mqtt::MqttMessage, mqtt_topics::MqttTopic};

/// Log a message to the console and send it to the MQTT broker
pub async fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::Info => info!("{}", message),
        LogLevel::Warn => warn!("{}", message),
        LogLevel::Error => error!("{}", message),
        LogLevel::Debug => debug!("{}", message),
    }
    MQTT_SEND
        .send(MqttMessage {
            topic: MqttTopic::Logs,
            payload: String::<512>::from_str(message).unwrap(),
        })
        .await;
}
