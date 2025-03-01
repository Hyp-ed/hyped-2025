use super::{can_receiver::INCOMING_STATE_TRANSITION_REQUESTS, can_sender::CAN_SEND};
use hyped_core::comms::{boards::Board, messages::CanMessage, state_transition::StateTransition};
use hyped_state_machine::state_machine::StateMachine;

use {defmt_rtt as _, panic_probe as _};

/// Handles the state machine logic by receiving state transition requests and sending new states.
#[embassy_executor::task]
pub async fn state_machine(board: Board) {
    // Initialise the state machine with the initial state
    let mut state_machine = StateMachine::new();

    let incoming_state_transition_requests = INCOMING_STATE_TRANSITION_REQUESTS.receiver();

    let can_sender = CAN_SEND.sender();

    loop {
        let state_transition = incoming_state_transition_requests.receive().await;
        let to_state = state_transition.to_state;

        let new_state = state_machine.handle_transition(&to_state);

        match new_state {
            Some(state) => {
                defmt::info!("State transition successful. New state: {:?}", state);

                // Send the new state to the CAN bus
                let can_message = CanMessage::StateTransition(StateTransition::new(board, state));
                can_sender.send(can_message).await;
            }
            None => {}
        }
    }
}
