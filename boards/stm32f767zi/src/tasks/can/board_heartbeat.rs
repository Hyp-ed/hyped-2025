use embassy_time::{with_timeout, Duration, Timer};
use hyped_communications::{
    boards::Board, emergency::Reason, heartbeat::Heartbeat, messages::CanMessage,
};
use hyped_core::config::HEARTBEAT_CONFIG;

use crate::{
    board_state::{EMERGENCY, THIS_BOARD},
    emergency,
    tasks::can::{receive::INCOMING_HEARTBEATS, send::CAN_SEND},
};

use defmt_rtt as _;
use panic_probe as _;

/// Task that listens for incoming heartbeat messages from the target board
/// and triggers an emergency stop if the target board does not respond in time.
/// For the controller boards, this should be spawned once for every other board.
/// For all other boards, this should be spawned once for the controller board.
#[embassy_executor::task]
pub async fn heartbeat_listener(from_board: Board) {
    match wait_for_first_heartbeat(from_board).await {
        Ok(_) => {
            defmt::info!("Board {:?} is alive!", from_board,);
        }
        Err(_) => {
            defmt::error!(
                "Failed to receive first heartbeat from board {:?}",
                from_board,
            );
            emergency!(Reason::NoInitialHeartbeat);
        }
    }

    loop {
        // Wait for an incoming heartbeat message from the target board
        match with_timeout(Duration::from_millis(HEARTBEAT_CONFIG.boards.max_latency_ms as u64), async {
            loop {
                // Only return when we receive a heartbeat message
                let heartbeat = INCOMING_HEARTBEATS.receive().await;
                if heartbeat.to == *THIS_BOARD.get().await && heartbeat.from == from_board {
                    defmt::debug!(
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
                emergency!(Reason::MissingHeartbeat);
            }
        }
    }
}

/// Gives the boards a chance to wake up at the start.
pub async fn wait_for_first_heartbeat(target_board: Board) -> Result<(), ()> {
    match with_timeout(
        Duration::from_secs(HEARTBEAT_CONFIG.boards.startup_timeout_s as u64),
        async {
            loop {
                // Only return when we receive a heartbeat message
                let heartbeat = INCOMING_HEARTBEATS.receive().await;
                if heartbeat.to == *THIS_BOARD.get().await && heartbeat.from == target_board {
                    break;
                }
            }
        },
    )
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
pub async fn send_heartbeat(to_board: Board) {
    let can_sender = CAN_SEND.sender();

    loop {
        // Send a heartbeat to the controller board every 100ms
        let heartbeat = Heartbeat::new(to_board, *THIS_BOARD.get().await);
        defmt::debug!("Sending heartbeat: {:?}", heartbeat);
        can_sender.send(CanMessage::Heartbeat(heartbeat)).await;

        Timer::after(Duration::from_hz(HEARTBEAT_CONFIG.boards.frequency as u64)).await;
    }
}
