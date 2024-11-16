// loop {
//     let envelope = rx.read().await.unwrap();
//     println!("Received: {:?}", envelope);
//     SEND_CHANNEL
//         .send(MqttMessage {
//             topic: MqttTopics::to_string(&MqttTopics::Debug),
//             payload: String::<512>::from_str(format!(&mut [0u8; 1024], "Received: {:?}", envelope).expect("invalid env")).unwrap(),
//         })
//         .await;
//     Timer::after(Duration::from_millis(100)).await;
// }

use super::mqtt::SEND_CHANNEL;
use core::str::FromStr;
use embassy_time::{Duration, Timer};
use heapless::String;
use hyped_core::{mqtt::MqttMessage, mqtt_topics::MqttTopics};
use embassy_stm32::can::{Can, CanRx, Fifo, StandardId, Frame, Id};
use static_cell::StaticCell;
use defmt::*;
use hyped_core::format;
use hyped_core::format_string::show;
use {defmt_rtt as _, panic_probe as _};

/// Sends a heartbeat message to the MQTT broker every second
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

        let topic_string = if *envelope.frame.header().id() == Id::from(StandardId::new(0).unwrap()) {
            MqttTopics::to_string(&MqttTopics::Debug)
        } else {
            MqttTopics::to_string(&MqttTopics::Test)
        };

        SEND_CHANNEL
            .send(MqttMessage {
                topic: topic_string,
                payload: String::<512>::from_str(format!(&mut [0u8; 1024], "Received: {:?}", envelope).expect("invalid env")).unwrap(),
            })
            .await;
        Timer::after(Duration::from_millis(100)).await;
    }
}
