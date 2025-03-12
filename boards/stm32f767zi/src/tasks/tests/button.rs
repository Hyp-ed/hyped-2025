use crate::tasks::mqtt::send::MQTT_SEND;
use core::str::FromStr;
use embassy_stm32::gpio::{AnyPin, Input, Pull};
use embassy_time::{Duration, Timer};
use heapless::String;
use hyped_core::{mqtt::MqttMessage, mqtt_topics::MqttTopic};
use serde::{Deserialize, Serialize};
use typenum::consts::*;
use {defmt_rtt as _, panic_probe as _};

/// Struct to hold the state of the button
#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonMqttMessage {
    pub status: bool,
}

/// Sends the state of the button over MQTT
/// (For testing purposes)
#[embassy_executor::task]
pub async fn button_task(pin: AnyPin) {
    let button = Input::new(pin, Pull::Down);
    loop {
        MQTT_SEND
            .send(MqttMessage {
                topic: MqttTopic::Debug,
                payload: String::<512>::from_str(
                    serde_json_core::to_string::<U512, ButtonMqttMessage>(&ButtonMqttMessage {
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
