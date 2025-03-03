use super::heartbeats_responder::INCOMING_HEARTBEATS;
use embassy_futures::join::join;
use embassy_stm32::can::{Can, CanRx, CanTx, ExtendedId, Frame, Id};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use hyped_can::HypedCanFrame;
use hyped_core::comms::{messages::CanMessage, state_transition::StateTransition};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
pub async fn can(mut can: Can<'static>) {
    defmt::info!("CAN enabled");
    join(can_receiver(can.split().1), can_sender(can.split().0)).await;
}

/// Stores incoming state transitions received from CAN.
/// All boards should listen to this channel and update their states accordingly.
pub static INCOMING_STATE_TRANSITIONS: Channel<CriticalSectionRawMutex, StateTransition, 10> =
    Channel::new();

/// Stores incoming state transition requests received from CAN.
/// Only used by the main control board running the state_machine task.
pub static INCOMING_STATE_TRANSITION_REQUESTS: Channel<
    CriticalSectionRawMutex,
    StateTransition,
    10,
> = Channel::new();

/// Task that receives CAN messages and puts them into a channel.
/// Currently only supports StateTransition and StateTransitionRequest messages.
async fn can_receiver(mut rx: CanRx<'static>) {
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
            CanMessage::StateTransition(state_transition) => {
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

/// Channel for sending CAN messages.
pub static CAN_SEND: Channel<CriticalSectionRawMutex, CanMessage, 10> = Channel::new();

/// Task that sends CAN messages from a channel.
async fn can_sender(mut tx: CanTx<'static>) {
    let can_sender = CAN_SEND.receiver();

    loop {
        let message = can_sender.receive().await;
        let can_frame: HypedCanFrame = message.into();

        let id = Id::Extended(ExtendedId::new(can_frame.can_id).unwrap());
        let data = can_frame.data;

        let frame = Frame::new_data(id, &data).unwrap();

        tx.write(&frame).await;

        Timer::after(Duration::from_millis(10)).await;
    }
}
