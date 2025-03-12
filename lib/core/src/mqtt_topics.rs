use core::str::FromStr;
use heapless::String;

/// Enum representing all MQTT topics used by the pod
#[derive(Debug, defmt::Format)]
pub enum MqttTopic {
    State,
    StateRequest,
    Accelerometer,
    OpticalFlow,
    Keyence,
    Displacement,
    Velocity,
    Acceleration,
    Heartbeat,
    Logs,
    Debug,
    Test,
}

impl MqttTopic {
    /// Convert an `MqttTopics` enum variant to a string
    pub fn to_string(&self) -> String<48> {
        match self {
            MqttTopic::State => String::<48>::from_str("hyped/pod_2025/state/state").unwrap(),
            MqttTopic::StateRequest => {
                String::<48>::from_str("hyped/pod_2025/state/state_request").unwrap()
            }
            MqttTopic::Accelerometer => {
                String::<48>::from_str("hyped/pod_2025/measurement/accelerometer").unwrap()
            }
            MqttTopic::OpticalFlow => {
                String::<48>::from_str("hyped/pod_2025/measurement/optical_flow").unwrap()
            }
            MqttTopic::Keyence => {
                String::<48>::from_str("hyped/pod_2025/measurement/keyence").unwrap()
            }
            MqttTopic::Displacement => {
                String::<48>::from_str("hyped/pod_2025/navigation/displacement").unwrap()
            }
            MqttTopic::Velocity => {
                String::<48>::from_str("hyped/pod_2025/navigation/velocity").unwrap()
            }
            MqttTopic::Acceleration => {
                String::<48>::from_str("hyped/pod_2025/navigation/acceleration").unwrap()
            }
            MqttTopic::Heartbeat => String::<48>::from_str("hyped/pod_2025/heartbeat").unwrap(),
            MqttTopic::Logs => String::<48>::from_str("hyped/pod_2025/logs").unwrap(),
            MqttTopic::Debug => String::<48>::from_str("debug").unwrap(),
            MqttTopic::Test => String::<48>::from_str("test").unwrap(),
        }
    }

    /// Get an `MqttTopics` enum variant from a string
    pub fn from_string(topic: &str) -> Option<MqttTopic> {
        match topic {
            "hyped/pod_2025/state/state" => Some(MqttTopic::State),
            "hyped/pod_2025/state/state_request" => Some(MqttTopic::StateRequest),
            "hyped/pod_2025/measurement/accelerometer" => Some(MqttTopic::Accelerometer),
            "hyped/pod_2025/measurement/optical_flow" => Some(MqttTopic::OpticalFlow),
            "hyped/pod_2025/measurement/keyence" => Some(MqttTopic::Keyence),
            "hyped/pod_2025/navigation/displacement" => Some(MqttTopic::Displacement),
            "hyped/pod_2025/navigation/velocity" => Some(MqttTopic::Velocity),
            "hyped/pod_2025/navigation/acceleration" => Some(MqttTopic::Acceleration),
            "hyped/pod_2025/heartbeat" => Some(MqttTopic::Heartbeat),
            "hyped/pod_2025/logs" => Some(MqttTopic::Logs),
            "debug" => Some(MqttTopic::Debug),
            "test" => Some(MqttTopic::Test),
            _ => None,
        }
    }
}
