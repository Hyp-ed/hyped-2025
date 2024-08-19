use core::str::FromStr;
use heapless::String;

pub enum MqttTopics {
    State,
    StateRequest,
    Accelerometer,
    OpticalFlow,
    Keyence,
    Displacement,
    Velocity,
    Acceleration,
    Logs,
}

// Write functions that will convert to and from the MqttTopics enum
impl MqttTopics {
    pub fn to_string(&self) -> String<48> {
        match self {
            MqttTopics::State => String::<48>::from_str("hyped/cart_2024/state/state").unwrap(),
            MqttTopics::StateRequest => {
                String::<48>::from_str("hyped/cart_2024/state/state_request").unwrap()
            }
            MqttTopics::Accelerometer => {
                String::<48>::from_str("hyped/cart_2024/measurement/accelerometer").unwrap()
            }
            MqttTopics::OpticalFlow => {
                String::<48>::from_str("hyped/cart_2024/measurement/optical_flow").unwrap()
            }
            MqttTopics::Keyence => {
                String::<48>::from_str("hyped/cart_2024/measurement/keyence").unwrap()
            }
            MqttTopics::Displacement => {
                String::<48>::from_str("hyped/cart_2024/navigation/displacement").unwrap()
            }
            MqttTopics::Velocity => {
                String::<48>::from_str("hyped/cart_2024/navigation/velocity").unwrap()
            }
            MqttTopics::Acceleration => {
                String::<48>::from_str("hyped/cart_2024/navigation/acceleration").unwrap()
            }
            MqttTopics::Logs => String::<48>::from_str("hyped/cart_2024/logs").unwrap(),
        }
    }

    pub fn from_string(topic: &str) -> Option<MqttTopics> {
        match topic {
            "hyped/cart_2024/state/state" => Some(MqttTopics::State),
            "hyped/cart_2024/state/state_request" => Some(MqttTopics::StateRequest),
            "hyped/cart_2024/measurement/accelerometer" => Some(MqttTopics::Accelerometer),
            "hyped/cart_2024/measurement/optical_flow" => Some(MqttTopics::OpticalFlow),
            "hyped/cart_2024/measurement/keyence" => Some(MqttTopics::Keyence),
            "hyped/cart_2024/navigation/displacement" => Some(MqttTopics::Displacement),
            "hyped/cart_2024/navigation/velocity" => Some(MqttTopics::Velocity),
            "hyped/cart_2024/navigation/acceleration" => Some(MqttTopics::Acceleration),
            "hyped/cart_2024/logs" => Some(MqttTopics::Logs),
            _ => None,
        }
    }
}