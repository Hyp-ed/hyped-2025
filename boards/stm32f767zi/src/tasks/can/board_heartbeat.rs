use embassy_time::{with_timeout, Duration, Timer};
use hyped_communications::{
    boards::Board, emergency::Reason, heartbeat::Heartbeat, messages::CanMessage,
};
use hyped_core::{format, format_string::show};

use crate::{
    emergency,
    tasks::can::{receive::INCOMING_HEARTBEATS, send::CAN_SEND},
};

use {defmt_rtt as _, panic_probe as _};

const HEARTBEAT_FREQUENCY: u64 = 5; // in Hz
const HEARTBEAT_MAX_LATENCY: u64 = 500; // in ms
const STARTUP_TIMEOUT: u64 = 30000; // in ms

/// Task that listens for incoming heartbeat messages from the target board
/// and triggers an emergency stop if the target board does not respond in time.
/// For the controller boards, this should be spawned once for every other board.
/// For all other boards, this should be spawned once for the controller board.
#[embassy_executor::task]
pub async fn heartbeat_listener(this_board: Board, from_board: Board) {
    wait_for_first_heartbeat(this_board, from_board)
        .await
        .expect(
            format!(
                &mut [0u8; 1024],
                "Failed to receive first heartbeat from board {:?}", from_board
            )
            .unwrap(),
        );

    loop {
        // Wait for an incoming heartbeat message from the target board
        match with_timeout(Duration::from_millis(HEARTBEAT_MAX_LATENCY), async {
            loop {
                // Only return when we receive a heartbeat message
                let heartbeat = INCOMING_HEARTBEATS.receive().await;
                if heartbeat.to == this_board && heartbeat.from == from_board {
                    defmt::info!(
                        "Received heartbeat from board {:?}",
                        from_board,
                    );
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
                    from_board
                );
                emergency!(this_board);
            }
        }
    }
}

/// Gives the boards a chance to wake up at the start.
pub async fn wait_for_first_heartbeat(this_board: Board, target_board: Board) -> Result<(), ()> {
    match with_timeout(Duration::from_millis(STARTUP_TIMEOUT), async {
        loop {
            // Only return when we receive a heartbeat message
            let heartbeat = INCOMING_HEARTBEATS.receive().await;
            if heartbeat.to == this_board && heartbeat.from == target_board {
                break;
            }
        }
    })
    .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

/// Sends heartbeats to the specified board.
/// For the controller board, this should be spawned once for every other board.
/// For all other boards, this should be spawned once for the controller board.
#[embassy_executor::task]
pub async fn send_heartbeat(this_board: Board, to_board: Board) {
    let can_sender = CAN_SEND.sender();

    loop {
        // Send a hearbeat to the controller board every 100ms
        let heartbeat = Heartbeat::new(to_board, this_board);
        defmt::info!("Sending heartbeat: {:?}", heartbeat);
        can_sender.send(CanMessage::Heartbeat(heartbeat)).await;
        Timer::after(Duration::from_hz(HEARTBEAT_FREQUENCY)).await;
    }
}
