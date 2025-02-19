use defmt::{info, warn};
use embassy_net::tcp::TcpSocket;
use heapless::String;
use rust_mqtt::{
    client::{
        client::MqttClient,
        client_config::{ClientConfig, MqttVersion},
    },
    packet::v5::{publish_packet::QualityOfService, reason_codes::ReasonCode},
    utils::rng_generator::CountingRng,
};

pub struct MqttMessage {
    pub topic: String<48>,
    pub payload: String<512>,
}

impl MqttMessage {
    pub fn new(topic: String<48>, payload: String<512>) -> Self {
        MqttMessage { topic, payload }
    }
}

pub struct HypedMqttClient<'a, T, R>
where
    T: embedded_io_async::Read + embedded_io_async::Write,
    R: rand_core::RngCore,
{
    client: MqttClient<'a, T, 5, R>,
}

impl<'a> HypedMqttClient<'a, TcpSocket<'a>, CountingRng> {
    /// Create a new HypedMqttClient instance with the given network configuration
    pub fn new(
        network_driver: TcpSocket<'a>,
        buffer: &'a mut [u8],
        buffer_len: usize,
        recv_buffer: &'a mut [u8],
        recv_buffer_len: usize,
        client_id: &'a str,
    ) -> Self {
        let config = initialise_mqtt_config(client_id);
        let client = MqttClient::new(
            network_driver,
            buffer,
            buffer_len,
            recv_buffer,
            recv_buffer_len,
            config,
        );

        HypedMqttClient { client }
    }
}

/// Initialise the MQTT client configuration with the given client ID
pub fn initialise_mqtt_config(client_id: &str) -> ClientConfig<'_, 5, CountingRng> {
    let mut config = ClientConfig::new(MqttVersion::MQTTv5, CountingRng(20000));
    config.add_max_subscribe_qos(QualityOfService::QoS1);
    config.add_client_id(client_id);
    config.max_packet_size = 100;
    config
}

// Implement send_message for HypedMqttClient
impl<T: embedded_io_async::Read + embedded_io_async::Write, R: rand_core::RngCore>
    HypedMqttClient<'_, T, R>
{
    pub async fn connect_to_broker(&mut self) {
        match self.client.connect_to_broker().await {
            Ok(()) => {}
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    info!("MQTT Network Error");
                }
                _ => {
                    warn!("Other MQTT Error: {:?}", mqtt_error);
                }
            },
        }
    }

    pub async fn send_message(&mut self, topic: &str, message: &[u8], retain: bool) {
        match self
            .client
            .send_message(topic, message, QualityOfService::QoS1, retain)
            .await
        {
            Ok(()) => {}
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    info!("MQTT Network Error");
                }
                _ => {
                    warn!("Other MQTT Error: {:?}", mqtt_error);
                }
            },
        }
    }

    pub async fn subscribe(&mut self, topic: &str) {
        match self.client.subscribe_to_topic(topic).await {
            Ok(()) => {}
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    info!("MQTT Network Error");
                }
                _ => {
                    warn!("Other MQTT Error: {:?}", mqtt_error);
                }
            },
        }
    }

    pub async fn receive_message(&mut self) -> Result<(&str, &str), ReasonCode> {
        match self.client.receive_message().await {
            Ok((topic, payload)) => {
                let payload_str = core::str::from_utf8(payload).unwrap();
                Ok((topic, payload_str))
            }
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    info!("MQTT Network Error");
                    Err(ReasonCode::NetworkError)
                }
                _ => {
                    warn!("Other MQTT Error: {:?}", mqtt_error);
                    Err(mqtt_error)
                }
            },
        }
    }
}
