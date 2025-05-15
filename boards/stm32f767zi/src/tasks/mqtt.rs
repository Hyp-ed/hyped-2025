pub mod base_station_heartbeat;
pub mod receive;
pub mod send;

use core::str::FromStr;
use embassy_futures::join::join;
use embassy_net::{Ipv4Address, Stack};
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
use receive::mqtt_receive;
use send::mqtt_send;

use crate::config::TELEMETRY;

/// Split up the CAN peripheral into a sender and receiver.
#[embassy_executor::task]
pub async fn mqtt(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    let mqtt_broker_address = (
        Ipv4Address::from_str(TELEMETRY.mqtt.broker.ip).expect("Invalid MQTT broker IP address"),
        TELEMETRY.mqtt.broker.port as u16,
    );
    join(
        mqtt_send(stack, mqtt_broker_address),
        mqtt_receive(stack, mqtt_broker_address),
    )
    .await;
}
