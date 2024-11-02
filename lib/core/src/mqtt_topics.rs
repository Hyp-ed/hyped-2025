use core::str::FromStr;
use heapless::String;

/// Enum representing all MQTT topics used by the pod
pub enum MqttTopics {
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

impl MqttTopics {
    /// Convert an `MqttTopics` enum variant to a string
    pub fn to_string(&self) -> String<48> {
        match self {
            MqttTopics::State => String::<48>::from_str("hyped/pod_2025/state/state").unwrap(),
            MqttTopics::StateRequest => {
                String::<48>::from_str("hyped/pod_2025/state/state_request").unwrap()
            }
            MqttTopics::Accelerometer => {
                String::<48>::from_str("hyped/pod_2025/measurement/accelerometer").unwrap()
            }
            MqttTopics::OpticalFlow => {
                String::<48>::from_str("hyped/pod_2025/measurement/optical_flow").unwrap()
            }
            MqttTopics::Keyence => {
                String::<48>::from_str("hyped/pod_2025/measurement/keyence").unwrap()
            }
            MqttTopics::Displacement => {
                String::<48>::from_str("hyped/pod_2025/navigation/displacement").unwrap()
            }
            MqttTopics::Velocity => {
                String::<48>::from_str("hyped/pod_2025/navigation/velocity").unwrap()
            }
            MqttTopics::Acceleration => {
                String::<48>::from_str("hyped/pod_2025/navigation/acceleration").unwrap()
            }
            MqttTopics::Heartbeat => String::<48>::from_str("hyped/pod_2025/heartbeat").unwrap(),
            MqttTopics::Logs => String::<48>::from_str("hyped/pod_2025/logs").unwrap(),
            MqttTopics::Debug => String::<48>::from_str("debug").unwrap(),
            MqttTopics::Test => String::<48>::from_str("test").unwrap(),
        }
    }

    /// Get an `MqttTopics` enum variant from a string
    pub fn from_string(topic: &str) -> Option<MqttTopics> {
        match topic {
            "hyped/pod_2025/state/state" => Some(MqttTopics::State),
            "hyped/pod_2025/state/state_request" => Some(MqttTopics::StateRequest),
            "hyped/pod_2025/measurement/accelerometer" => Some(MqttTopics::Accelerometer),
            "hyped/pod_2025/measurement/optical_flow" => Some(MqttTopics::OpticalFlow),
            "hyped/pod_2025/measurement/keyence" => Some(MqttTopics::Keyence),
            "hyped/pod_2025/navigation/displacement" => Some(MqttTopics::Displacement),
            "hyped/pod_2025/navigation/velocity" => Some(MqttTopics::Velocity),
            "hyped/pod_2025/navigation/acceleration" => Some(MqttTopics::Acceleration),
            "hyped/pod_2025/heartbeat" => Some(MqttTopics::Heartbeat),
            "hyped/pod_2025/logs" => Some(MqttTopics::Logs),
            "debug" => Some(MqttTopics::Debug),
            "test" => Some(MqttTopics::Test),
            _ => None,
        }
    }
}
