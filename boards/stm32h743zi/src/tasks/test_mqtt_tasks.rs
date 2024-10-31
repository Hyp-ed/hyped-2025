use crate::log::log;
use core::str::FromStr;
use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_time::{Duration, Timer};
use heapless::String;
use hyped_core::{log_types::LogLevel, mqtt::MqttMessage, mqtt_topics::MqttTopics};
use serde::{Deserialize, Serialize};
use typenum::consts::*;

use super::mqtt_send_and_recv::SEND_CHANNEL;

use {defmt_rtt as _, panic_probe as _};

/// Struct to hold the state of the button
#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonMqttMessage {
    pub task_id: u8,
    pub status: bool,
}

/// Sends the state of the button over MQTT
#[embassy_executor::task]
pub async fn button_task(pin: AnyPin) {
    let button = Input::new(pin, Pull::Down);
    loop {
        SEND_CHANNEL
            .send(MqttMessage {
                topic: MqttTopics::to_string(&MqttTopics::Debug),
                payload: String::<512>::from_str(
                    serde_json_core::to_string::<U512, ButtonMqttMessage>(&ButtonMqttMessage {
                        task_id: 0,
                        status: button.is_high(),
                    })
                    .unwrap()
                    .as_str(),
                )
                .unwrap(),
            })
            .await;
        Timer::after(Duration::from_millis(100)).await;
    }
}

/// Sends a ping MQTT message every five seconds
#[embassy_executor::task]
pub async fn five_seconds_task() {
    loop {
        log(LogLevel::Info, "Ping from five second loop").await;
        SEND_CHANNEL
            .send(MqttMessage {
                topic: MqttTopics::to_string(&MqttTopics::Debug),
                payload: String::<512>::from_str(
                    serde_json_core::to_string::<U512, ButtonMqttMessage>(&ButtonMqttMessage {
                        task_id: 2,
                        status: false,
                    })
                    .unwrap()
                    .as_str(),
                )
                .unwrap(),
            })
            .await;
        Timer::after(Duration::from_secs(5)).await;
    }
}
