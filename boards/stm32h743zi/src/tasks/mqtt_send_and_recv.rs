use crate::{log::log, config::MQTT_BROKER_ADDRESS};

use embassy_net::{tcp::TcpSocket, Ipv4Address, Stack};
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

/// Channel for sending messages to the MQTT broker
pub static SEND_CHANNEL: Channel<ThreadModeRawMutex, MqttMessage, 128> = Channel::new();

/// Task for receiving messages from the MQTT broker
#[embassy_executor::task]
pub async fn mqtt_recv_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    let mut rx_buffer: [u8; 4096] = [0; 4096];
    let mut tx_buffer: [u8; 4096] = [0; 4096];
    let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(600)));
    log(LogLevel::Info, "Connecting to Receive Socket...").await;
    match socket
        .connect(MQTT_BROKER_ADDRESS)
        .await
    {
        Ok(()) => {
            log(LogLevel::Info, "Connected to Receive!").await;
        }
        Err(connection_error) => {
            log(
                LogLevel::Error,
                format!(&mut [0u8; 1024], "Error connecting: {:?}", connection_error).unwrap(),
            )
            .await;
        }
    };
    let mut recv_buffer = [0; 1024];
    let mut write_buffer = [0; 1024];
    let mut mqtt_client = HypedMqttClient::new(
        socket,
        &mut write_buffer,
        1024,
        &mut recv_buffer,
        1024,
        "receiver-stm-client",
    );
    mqtt_client.connect_to_broker().await;

    mqtt_client.subscribe("command_sender").await;
    mqtt_client.subscribe("acceleration").await;

    loop {
        match mqtt_client.receive_message().await {
            Ok((topic, message)) => {
                log(
                    LogLevel::Info,
                    format!(
                        &mut [0u8; 1024],
                        "Received message on topic {}: {}", topic, message
                    )
                    .unwrap(),
                )
                .await
            }
            Err(err) => {
                if err == rust_mqtt::packet::v5::reason_codes::ReasonCode::NetworkError {
                    break;
                }
                log(
                    LogLevel::Error,
                    format!(&mut [0u8; 1024], "Error receiving message: {:?}", err).unwrap(),
                )
                .await
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

/// Task for sending messages from `SEND_CHANNEL` to the MQTT broker
#[embassy_executor::task]
pub async fn mqtt_send_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    let mut rx_buffer: [u8; 4096] = [0; 4096];
    let mut tx_buffer: [u8; 4096] = [0; 4096];
    let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(60)));
    log(LogLevel::Info, "Connecting to Send Socket...").await;
    match socket
        .connect(MQTT_BROKER_ADDRESS)
        .await
    {
        Ok(()) => log(LogLevel::Info, "Connected to Send!").await,
        Err(connection_error) => {
            log(
                LogLevel::Error,
                format!(&mut [0u8; 1024], "Error connecting: {:?}", connection_error).unwrap(),
            )
            .await;
        }
    };

    let mut recv_buffer = [0; 1024];
    let mut write_buffer = [0; 1024];
    let mut mqtt_client = HypedMqttClient::new(
        socket,
        &mut write_buffer,
        1024,
        &mut recv_buffer,
        1024,
        "sender-stm-client",
    );

    mqtt_client.connect_to_broker().await;

    loop {
        while !SEND_CHANNEL.is_empty() {
            let message = SEND_CHANNEL.receive().await;

            mqtt_client
                .send_message(message.topic.as_str(), message.payload.as_bytes(), false)
                .await;
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

/// Task for running the network stack
#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) -> ! {
    stack.run().await
}
