use super::heartbeats_responder::INCOMING_HEARTBEATS;
use embassy_stm32::can::{Can, CanRx, Id};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use hyped_can::HypedCanFrame;
use hyped_communications::{messages::CanMessage, state_transition::StateTransitionRequest};

use {defmt_rtt as _, panic_probe as _};

/// Stores incoming state transitions received from CAN.
/// All boards should listen to this channel and update their states accordingly.
pub static INCOMING_STATE_TRANSITIONS: Channel<
    CriticalSectionRawMutex,
    StateTransitionRequest,
    10,
> = Channel::new();

/// Stores incoming state transition requests received from CAN.
/// Only used by the main control board running the state_machine task.
pub static INCOMING_STATE_TRANSITION_REQUESTS: Channel<
    CriticalSectionRawMutex,
    StateTransitionRequest,
    10,
> = Channel::new();

/// Stores heartbeat messages coming in from other boards that we need to respond to.
pub static INCOMING_HEARTBEATS: Channel<CriticalSectionRawMutex, Heartbeat, 10> = Channel::new();

/// Task that receives CAN messages and puts them into a channel.
/// Currently only supports `StateTransitionCommand`, `StateTransitionRequest` and `Heartbeat` messages.
pub async fn can_receiver(mut rx: CanRx<'static>) {
    let state_transition_sender = INCOMING_STATE_TRANSITIONS.sender();
    let state_transition_request_sender = INCOMING_STATE_TRANSITION_REQUESTS.sender();
    let incoming_heartbeat_sender = INCOMING_HEARTBEATS.sender();

    loop {
        defmt::info!("Waiting for CAN message");

        let envelope = rx.read().await;
        if envelope.is_err() {
            continue;
        }
        let envelope = envelope.unwrap();
        let id = envelope.frame.id();
        let can_id = match id {
            Id::Standard(id) => id.as_raw() as u32, // 11-bit ID
            Id::Extended(id) => id.as_raw(),        // 29-bit ID
        };
        let mut data = [0u8; 8];
        data.copy_from_slice(envelope.frame.data());
        let can_frame = HypedCanFrame::new(can_id, data);

        let can_message: CanMessage = can_frame.into();

        match can_message {
            CanMessage::StateTransitionCommand(state_transition) => {
                state_transition_sender.send(state_transition).await;
            }
            // Requests will only be used on the primary board running the state_machine task.
            CanMessage::StateTransitionRequest(state_transition) => {
                state_transition_request_sender.send(state_transition).await;
            }
            CanMessage::Heartbeat(heartbeat) => {
                defmt::info!("Received heartbeat: {:?}", heartbeat);
                incoming_heartbeat_sender.send(heartbeat).await;
            }
            _ => {}
        }

        Timer::after(Duration::from_millis(10)).await;
    }
}
