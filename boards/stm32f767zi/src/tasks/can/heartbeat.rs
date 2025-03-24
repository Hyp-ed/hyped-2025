use embassy_time::{with_timeout, Duration, Timer};
use hyped_communications::{
    boards::Board, emergency::Reason, heartbeat::Heartbeat, messages::CanMessage,
};

use crate::{
    emergency,
    tasks::can::{receive::INCOMING_HEARTBEATS, send::CAN_SEND},
};

use {defmt_rtt as _, panic_probe as _};

static HEARTBEAT_FREQUENCY: u64 = 10; // in Hz
static HEARTBEAT_MAX_LATENCY: u64 = 500; // in ms

/// Task that sends heartbeats to other boards and checks if they are still alive.
/// If a board does not respond in time, an emergency stop is triggered.
#[embassy_executor::task]
pub async fn heartbeat_controller(this_board: Board, target_board: Board) {
    // Send initial messages
    let heartbeat = Heartbeat::new(target_board, this_board);
    defmt::info!("Sending initial heartbeat: {:?}", heartbeat);
    CAN_SEND.send(CanMessage::Heartbeat(heartbeat)).await;

    loop {
        // Wait for an incoming heartbeat message from the target board
        match with_timeout(Duration::from_hz(HEARTBEAT_MAX_LATENCY), async {
            loop {
                // Only return when we receive a heartbeat message
                let heartbeat = INCOMING_HEARTBEATS.receive().await;
                if heartbeat.to == this_board && heartbeat.from == target_board {
                    break;
                }
            }
        })
        .await
        // trigger emergency stop if we don't receive a heartbeat in time
        {
            Ok(_) => {}
            Err(_) => {
                defmt::error!(
                    "Emergency stop triggered due to missing heartbeat from board {:?}",
                    target_board
                );
                emergency!(this_board);
            }
        }

        Timer::after(Duration::from_hz(HEARTBEAT_FREQUENCY)).await;
    }
}

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
