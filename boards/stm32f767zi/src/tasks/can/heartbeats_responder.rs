use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use hyped_core::comms::{boards::Board, heartbeat::Heartbeat, messages::CanMessage};

use super::send::CAN_SEND;

use {defmt_rtt as _, panic_probe as _};

/// Stores heartbeat messages coming in from other boards that we need to respond to.
/// This is populated by the CAN receiver task.
pub static INCOMING_HEARTBEATS: Channel<CriticalSectionRawMutex, Heartbeat, 10> = Channel::new();

/// Task that responds to incoming heartbeat messages
#[embassy_executor::task]
pub async fn heartbeat_responder(this_board: Board) {
    let can_sender = CAN_SEND.sender();

    loop {
        defmt::info!("Waiting...");

        // Wait for an incoming heartbeat message
        let heartbeat = INCOMING_HEARTBEATS.receive().await;
        if heartbeat.to == this_board {
            // We received a heartbeat message meant for us, so we should respond to it
            // Send it back!
            defmt::info!("Responding to heartbeat from {:?}", heartbeat.from);
            let heartbeat_response = Heartbeat::new(heartbeat.from, this_board);
            can_sender
                .send(CanMessage::Heartbeat(heartbeat_response))
                .await;
        }
        // Otherwise, ignore the message

        Timer::after(Duration::from_millis(10)).await;
    }
}
