use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use hyped_communications::{
    boards::Board, heartbeat::Heartbeat, messages::CanMessage, state_transition::StateTransition,
};
use hyped_state_machine::states::State;

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
    // Keep track of every board's status
    let mut last_ack = 0;
    let mut latency = 0;

    // Send initial messages
    let heartbeat = Heartbeat::new(target_board, this_board);
    defmt::info!("Sending initial heartbeat: {:?}", heartbeat);
    CAN_SEND.send(CanMessage::Heartbeat(heartbeat)).await;

    loop {
        // Wait for an incoming heartbeat message from the target board
        with_timeout(Duration::from_hz(HEARTBEAT_MAX_LATENCY), {
            loop {
                // Only return when we receive a heartbeat message
                let heartbeat = INCOMING_HEARTBEATS.receive().await;
                if heartbeat.to == this_board && heartbeat.from == target_board {
                    return;
                }
            }
        })
        .await
        // trigger emergency stop if we don't receive a heartbeat in time
        .unwrap_or_else(async |_| {
            defmt::error!(
                "Emergency stop triggered due to missing heartbeat from board {:?}",
                target_board
            );
            emergency!(this_board);
        });

        // Update the last time we received a heartbeat from the target board
        latency = Instant::now().as_millis() - last_ack;
        last_ack = Instant::now().as_millis();

        Timer::after(Duration::from_hz(HEARTBEAT_FREQUENCY)).await;
    }
}
