use crate::tasks::mqtt_send_and_recv::SEND_CHANNEL;
use core::str::FromStr;
use defmt::{debug, error, info, warn};
use heapless::String;
use hyped_core::{log_types::LogLevel, mqtt::MqttMessage, mqtt_topics::MqttTopics};

/// Log a message to the console and send it to the MQTT broker
pub async fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::Info => info!("{}", message),
        LogLevel::Warn => warn!("{}", message),
        LogLevel::Error => error!("{}", message),
        LogLevel::Debug => debug!("{}", message),
    }
    SEND_CHANNEL
        .send(MqttMessage {
            topic: MqttTopics::to_string(&MqttTopics::Logs),
            payload: String::<512>::from_str(message).unwrap(),
        })
        .await;
}
