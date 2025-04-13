// TODOLater: pod name

use core::str::FromStr;
use heapless::String;

use crate::config::MeasurementId;

pub const MQTT_MEASUREMENT_TOPIC_PREFIX: &str = "hyped/poddington/measurement/";

/// Enum representing all MQTT topics used by the pod
#[derive(Debug, defmt::Format, PartialEq, Eq)]
pub enum MqttTopic {
    Measurement(MeasurementId),
    State,
    StateRequest,
    Heartbeat,
    Logs,
    Debug,
    Test,
}

impl MqttTopic {
    /// Convert an `MqttTopics` enum variant to a string
    pub fn to_string(&self) -> String<100> {
        match self {
            MqttTopic::State => String::<100>::from_str("hyped/poddington/state/state").unwrap(),
            MqttTopic::StateRequest => {
                String::<100>::from_str("hyped/podpoddington_2025/state/state_request").unwrap()
            }
            MqttTopic::Heartbeat => String::<100>::from_str("hyped/poddington/heartbeat").unwrap(),
            MqttTopic::Logs => String::<100>::from_str("hyped/poddington/logs").unwrap(),
            MqttTopic::Debug => String::<100>::from_str("debug").unwrap(),
            MqttTopic::Test => String::<100>::from_str("test").unwrap(),
            MqttTopic::Measurement(measurement_id) => {
                let measurement_id_string: String<50> = (*measurement_id).into();
                let mut topic = String::<100>::from_str("hyped/poddington/measurement/").unwrap();
                topic.push_str(measurement_id_string.as_str()).unwrap();
                topic
            }
        }
    }

    /// Get an `MqttTopics` enum variant from a string
    pub fn from_string(topic: &str) -> Option<MqttTopic> {
        match topic {
            "hyped/poddington/state/state" => Some(MqttTopic::State),
            "hyped/poddington/state/state_request" => Some(MqttTopic::StateRequest),
            "hyped/poddington/heartbeat" => Some(MqttTopic::Heartbeat),
            "hyped/poddington/logs" => Some(MqttTopic::Logs),
            "debug" => Some(MqttTopic::Debug),
            "test" => Some(MqttTopic::Test),
            _ => {
                if topic.starts_with(MQTT_MEASUREMENT_TOPIC_PREFIX) {
                    let measurement_id_string =
                        &topic[MQTT_MEASUREMENT_TOPIC_PREFIX.len()..topic.len()];
                    let measurement_id = measurement_id_string.into();
                    Some(MqttTopic::Measurement(measurement_id))
                } else {
                    None
                }
            }
        }
    }
}

impl FromStr for MqttTopic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match MqttTopic::from_string(s) {
            Some(topic) => Ok(topic),
            None => {
                defmt::error!("Failed to parse MQTT topic: {}", s);
                Err(())
            }
        }
    }
}
