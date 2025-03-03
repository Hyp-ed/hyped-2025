use crate::emergency;

use super::can::CAN_SEND;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Instant, Timer};
use hyped_core::{
    comms::{
        boards::Board, heartbeat::Heartbeat, messages::CanMessage,
        state_transition::StateTransition,
    },
    states::State,
};

use {defmt_rtt as _, panic_probe as _};

/// Stores heartbeat messages coming in from other boards that we need to respond to.
/// This is populated by the CAN receiver task.
pub static INCOMING_HEARTBEATS: Channel<CriticalSectionRawMutex, Heartbeat, 10> = Channel::new();

static MAX_HEARTBEAT_DELAY: u64 = 1000;

/// Task that responds to incoming heartbeat messages.
#[embassy_executor::task]
pub async fn heartbeat_coordinator(this_board: Board) {
    // Keep track of every board's status
    let mut keyence_tester_last_ack = 0;

    // Send initial messages
    let heartbeat = Heartbeat::new(Board::KeyenceTester, this_board);
    defmt::info!("Sending initial heartbeat: {:?}", heartbeat);
    CAN_SEND.send(CanMessage::Heartbeat(heartbeat)).await;

    loop {
        // Wait for an incoming heartbeat message
        let heartbeat = INCOMING_HEARTBEATS.receive().await;
        if heartbeat.to == this_board {
            match heartbeat.from {
                Board::KeyenceTester => {
                    defmt::info!("Received heartbeat from KeyenceTester");
                    keyence_tester_last_ack = Instant::now().as_millis();
                }
                _ => {}
            }
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}

async fn check_heartbeats(this_board: Board) {
    // Keep track of every board's status
    let mut keyence_tester_last_ack = 0;

    let can_sender = CAN_SEND.sender();

    loop {
        // Check if we need to send a new heartbeat message
        let now = Instant::now().as_millis();
        if now - keyence_tester_last_ack > MAX_HEARTBEAT_DELAY / 2 {
            defmt::info!("Sending heartbeat");
            let heartbeat = Heartbeat::new(this_board, Board::KeyenceTester);
            CAN_SEND.send(CanMessage::Heartbeat(heartbeat)).await;
        }

        // Check if any boards have not replied in MAX_HEARTBEAT_DELAY milliseconds
        let now = Instant::now().as_millis();
        if now - keyence_tester_last_ack > MAX_HEARTBEAT_DELAY {
            emergency!(can_sender, this_board);
        }
    }
}
