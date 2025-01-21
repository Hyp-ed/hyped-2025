use super::mqtt_send::SEND_TO_MQTT_CHANNEL;
use core::str::FromStr;
use defmt::*;
use embassy_stm32::can::{CanRx, Id, StandardId};
use embassy_time::{Duration, Timer};
use heapless::String;
use hyped_core::format;
use hyped_core::format_string::show;
use hyped_core::{mqtt::MqttMessage, mqtt_topics::MqttTopics};
use {defmt_rtt as _, panic_probe as _};

/// Receives CAN messages and sends them over MQTT to the base station.
/// The CAN messages can be sent to different topics based on the CAN ID.
#[embassy_executor::task]
pub async fn can_receiver(rx: &'static mut CanRx<'static>) {
    Timer::after(Duration::from_secs(1)).await;
    loop {
        let envelope = rx.read().await;
        if envelope.is_err() {
            continue;
        }
        let envelope = envelope.unwrap();
        println!("Received: {:?}", envelope);

        let topic_string = if *envelope.frame.header().id() == Id::from(StandardId::new(0).unwrap())
        {
            MqttTopics::to_string(&MqttTopics::Debug)
        } else {
            MqttTopics::to_string(&MqttTopics::Test)
        };

        SEND_TO_MQTT_CHANNEL
            .send(MqttMessage {
                topic: topic_string,
                payload: String::<512>::from_str(
                    format!(&mut [0u8; 1024], "Received: {:?}", envelope).expect("invalid env"),
                )
                .unwrap(),
            })
            .await;
        Timer::after(Duration::from_millis(100)).await;
    }
}
