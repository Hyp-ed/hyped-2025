use crate::{log::log, telemetry_config::MQTT_BROKER_ADDRESS};
use embassy_net::{tcp::TcpSocket, Stack};
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use hyped_core::{
    format,
    format_string::show,
    log_types::LogLevel,
    mqtt::{HypedMqttClient, MqttMessage},
};
use {defmt_rtt as _, panic_probe as _};

/// Channel for sending messages over MQTT.
/// Any message sent to this channel will be sent to the MQTT broker by the `mqtt_send_task`
pub static SEND_TO_MQTT_CHANNEL: Channel<ThreadModeRawMutex, MqttMessage, 128> = Channel::new();

/// Task for sending messages from `SEND_CHANNEL` to the MQTT broker
#[embassy_executor::task]
pub async fn mqtt_send_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    let mut rx_buffer: [u8; 4096] = [0; 4096];
    let mut tx_buffer: [u8; 4096] = [0; 4096];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(60)));

    log(LogLevel::Info, "Connecting to Send Socket...").await;

    match socket.connect(MQTT_BROKER_ADDRESS).await {
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
        "telemetry_board_sender",
    );

    mqtt_client.connect_to_broker().await;

    loop {
        while !SEND_TO_MQTT_CHANNEL.is_empty() {
            let message = SEND_TO_MQTT_CHANNEL.receive().await;
            mqtt_client
                .send_message(message.topic.as_str(), message.payload.as_bytes(), false)
                .await;
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
