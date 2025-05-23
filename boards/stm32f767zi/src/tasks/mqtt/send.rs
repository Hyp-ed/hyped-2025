use core::str::FromStr;

use crate::log::log;
use defmt_rtt as _;
use embassy_net::{tcp::TcpSocket, Ipv4Address, Stack};
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use heapless::String;
use hyped_core::{
    config::TELEMETRY_CONFIG,
    format,
    format_string::show,
    log_types::LogLevel,
    mqtt::{HypedMqttClient, MqttMessage},
    mqtt_topics::MqttTopic,
};
use panic_probe as _;

/// Channel for sending messages over MQTT.
/// Any message sent to this channel will be sent to the MQTT broker by the `mqtt_send_task`
pub static MQTT_SEND: Channel<ThreadModeRawMutex, MqttMessage, 128> = Channel::new();

/// Sends messages from `SEND_CHANNEL` to the MQTT broker
pub async fn mqtt_send(
    stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>,
    mqtt_broker_address: (Ipv4Address, u16),
) {
    let mut rx_buffer: [u8; 4096] = [0; 4096];
    let mut tx_buffer: [u8; 4096] = [0; 4096];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(60)));

    log(LogLevel::Info, "Connecting to Send Socket...").await;

    match socket.connect(mqtt_broker_address).await {
        Ok(()) => log(LogLevel::Info, "Connected to Send!").await,
        Err(connection_error) => {
            log(
                LogLevel::Error,
                format!(&mut [0u8; 1024], "Error connecting: {:?}", connection_error).unwrap(),
            )
            .await;
        }
    };

    const RECV_BUFFER_LEN: usize = 1024;
    const WRITE_BUFFER_LEN: usize = 1024;
    let mut recv_buffer = [0; RECV_BUFFER_LEN];
    let mut write_buffer = [0; WRITE_BUFFER_LEN];

    let mut mqtt_client = HypedMqttClient::new(
        socket,
        &mut write_buffer,
        RECV_BUFFER_LEN,
        &mut recv_buffer,
        WRITE_BUFFER_LEN,
        TELEMETRY_CONFIG.mqtt.sender.client_id,
    );

    mqtt_client.connect_to_broker().await;

    defmt::info!("Connected to MQTT broker");

    MQTT_SEND
        .send(MqttMessage::new(
            MqttTopic::Test,
            String::from_str("Hello from telemetry board!").unwrap(),
        ))
        .await;

    loop {
        let message = MQTT_SEND.receive().await;
        defmt::debug!("Sending MQTT message: {}", message);
        let topic_string: String<100> = message.topic.into();
        mqtt_client
            .send_message(topic_string.as_str(), message.payload.as_bytes(), false)
            .await;
    }
}
