// TODOLater: pod name

use core::str::FromStr;
use heapless::String;

use crate::config::MeasurementId;

/// Enum representing all MQTT topics used by the pod
#[derive(Debug, defmt::Format)]
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
    pub fn to_string(&self) -> String<48> {
        match self {
            MqttTopic::State => String::<48>::from_str("hyped/poddington/state/state").unwrap(),
            MqttTopic::StateRequest => {
                String::<48>::from_str("hyped/podpoddington_2025/state/state_request").unwrap()
            }
            MqttTopic::Heartbeat => String::<48>::from_str("hyped/poddington/heartbeat").unwrap(),
            MqttTopic::Logs => String::<48>::from_str("hyped/poddington/logs").unwrap(),
            MqttTopic::Debug => String::<48>::from_str("debug").unwrap(),
            MqttTopic::Test => String::<48>::from_str("test").unwrap(),
            MqttTopic::Measurement(measurement_id) => {
                let measurement_id_string = measurement_id.to_string();
                let mut topic = String::<48>::from_str("hyped/poddington/measurement/").unwrap();
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
                if topic.starts_with("hyped/poddington/measurement/") {
                    let measurement_id_string = &topic[26..];
                    let measurement_id = MeasurementId::from_string(measurement_id_string);
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
            None => Err(()),
        }
    }
}
