use core::str::FromStr;

#[cfg(not(feature = "std"))]
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
    #[cfg(not(feature = "std"))]
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

    #[cfg(feature = "std")]
    pub fn to_string(&self) -> String {
        match self {
            MqttTopics::State => "hyped/cart_2024/state/state".to_string(),
            MqttTopics::StateRequest => "hyped/cart_2024/state/state_request".to_string(),
            MqttTopics::Accelerometer => "hyped/cart_2024/measurement/accelerometer".to_string(),
            MqttTopics::OpticalFlow => "hyped/cart_2024/measurement/optical_flow".to_string(),
            MqttTopics::Keyence => "hyped/cart_2024/measurement/keyence".to_string(),
            MqttTopics::Displacement => "hyped/cart_2024/navigation/displacement".to_string(),
            MqttTopics::Velocity => "hyped/cart_2024/navigation/velocity".to_string(),
            MqttTopics::Acceleration => "hyped/cart_2024/navigation/acceleration".to_string(),
            MqttTopics::Logs => "hyped/cart_2024/logs".to_string(),
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
