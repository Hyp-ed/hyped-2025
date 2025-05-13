use crate::{log::log, telemetry_config::MQTT_BROKER_ADDRESS};
use core::str::FromStr;
use embassy_net::{tcp::TcpSocket, Stack};
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use heapless::String;
use hyped_core::{
    format,
    format_string::show,
    log_types::LogLevel,
    mqtt::{HypedMqttClient, MqttMessage},
    mqtt_topics::MqttTopic,
};
use {defmt_rtt as _, panic_probe as _};

/// Channel containing messages that have been received from the MQTT broker.
/// This channel is populated by the `mqtt_recv_task` and can be consumed by other tasks.
/// Note: excludes heartbeat and log messages
pub static MQTT_RECEIVE: Channel<ThreadModeRawMutex, MqttMessage, 128> = Channel::new();

/// Receives messages from the MQTT broker and sends them to the `MQTT_RECEIVE` channel.
pub async fn mqtt_receive(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    let mut rx_buffer: [u8; 4096] = [0; 4096];
    let mut tx_buffer: [u8; 4096] = [0; 4096];
    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);

    log(LogLevel::Info, "Connecting to Receive Socket...").await;

    match socket.connect(MQTT_BROKER_ADDRESS).await {
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
        "telemetry_board_receiver",
    );

    mqtt_client.connect_to_broker().await;
    mqtt_client.subscribe("hyped/pod_2025/#").await;

    loop {
        match mqtt_client.receive_message().await {
            Ok((topic_str, message)) => {
                let topic: Result<MqttTopic, &str> = topic_str.parse();

                match topic {
                    // Ignore heartbeat and log messages
                    Ok(MqttTopic::Heartbeat) => {}
                    Ok(MqttTopic::Logs) => {}
                    Ok(topic) => {
                        // Send message to channel so that it can be consumed by other tasks
                        MQTT_RECEIVE
                            .send(MqttMessage::new(
                                topic,
                                String::from_str(message)
                                    .expect("Failed to convert message to string"),
                            ))
                            .await;
                    }
                    Err(_) => {
                        // Log warning for unknown topic
                        log(
                            LogLevel::Warn,
                            format!(
                                &mut [0u8; 1024],
                                "Received message on unknown topic {}: {}", topic_str, message
                            )
                            .unwrap(),
                        )
                        .await
                    }
                }
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
    }
}
