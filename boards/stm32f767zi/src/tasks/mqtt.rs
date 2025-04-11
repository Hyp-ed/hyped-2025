pub mod heartbeat;
pub mod receive;
pub mod send;

use embassy_futures::join::join;
use embassy_net::Stack;
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
use receive::mqtt_receive;
use send::mqtt_send;

/// Split up the CAN peripheral into a sender and receiver.
#[embassy_executor::task]
pub async fn mqtt(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    join(mqtt_send(stack), mqtt_receive(stack)).await;
}
