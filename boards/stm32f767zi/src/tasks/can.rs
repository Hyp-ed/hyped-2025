pub mod heartbeat_controller;
pub mod heartbeats_responder;
pub mod receive;
pub mod send;

use embassy_futures::join::join;
use embassy_stm32::can::Can;
use receive::can_receiver;
use send::can_sender;

/// Split up the CAN peripheral into a sender and receiver.
#[embassy_executor::task]
pub async fn can(mut can: Can<'static>) {
    defmt::info!("CAN enabled");
    join(can_receiver(can.split().1), can_sender(can.split().0)).await;
}
