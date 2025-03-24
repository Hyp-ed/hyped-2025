use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use hyped_communications::{boards::Board, heartbeat::Heartbeat, messages::CanMessage};

use super::{receive::INCOMING_HEARTBEATS, send::CAN_SEND};

use {defmt_rtt as _, panic_probe as _};

/// Task that responds to incoming heartbeat messages
#[embassy_executor::task]
pub async fn heartbeat_responder(this_board: Board) {
    let can_sender = CAN_SEND.sender();

    loop {
        // Wait for an incoming heartbeat message
        let heartbeat = INCOMING_HEARTBEATS.receive().await;
        if heartbeat.to == this_board {
            // We received a heartbeat message meant for us, so we should respond to it
            defmt::debug!("Responding to heartbeat from {:?}", heartbeat.from);
            can_sender
                .send(CanMessage::Heartbeat(Heartbeat::new(
                    heartbeat.from,
                    this_board,
                )))
                .await;
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}
