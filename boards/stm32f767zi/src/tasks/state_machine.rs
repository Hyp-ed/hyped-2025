use super::can::receive::INCOMING_STATE_TRANSITION_COMMANDS;
use crate::{
    board_state::{CURRENT_STATE, THIS_BOARD},
    tasks::can::{receive::INCOMING_STATE_TRANSITION_REQUESTS, send::CAN_SEND},
};
use hyped_communications::{messages::CanMessage, state_transition::StateTransitionCommand};
use hyped_state_machine::state_machine::StateMachine;

use defmt_rtt as _;
use panic_probe as _;

/// Handles the state machine logic by receiving state transition requests and sending new states.
/// Should only be run on one board.
#[embassy_executor::task]
pub async fn state_machine() {
    // Initialise the state machine with the initial state
    let mut state_machine = StateMachine::new();

    let state_sender = CURRENT_STATE.sender();

    let incoming_state_transition_requests = INCOMING_STATE_TRANSITION_REQUESTS.receiver();
    let can_sender = CAN_SEND.sender();

    loop {
        let state_transition = incoming_state_transition_requests.receive().await;
        let to_state = state_transition.to_state;

        let new_state = state_machine.handle_transition(&to_state);

        match new_state {
            Some(state) => {
                defmt::info!("State transition successful. New state: {:?}", state);

                // Update this board's state
                state_sender.send(state);

                // Send the new state to the CAN bus
                let can_message = CanMessage::StateTransitionCommand(StateTransitionCommand::new(
                    *THIS_BOARD.get().await,
                    state,
                ));
                can_sender.send(can_message).await;
            }
            None => {
                defmt::error!(
                    "State transition failed. Invalid transition from {:?} to {:?}",
                    state_machine.current_state,
                    to_state
                );
            }
        }
    }
}

/// Task that updates the current state of the system by receiving state transitions from the CAN.
/// Should be run on all boards except the one running the state machine task.
#[embassy_executor::task]
pub async fn state_updater() {
    let state_updater = CURRENT_STATE.sender();
    let incoming_state_transitions = INCOMING_STATE_TRANSITION_COMMANDS.receiver();

    loop {
        let state_transition = incoming_state_transitions.receive().await;
        defmt::info!("Changing state: {:?}", state_transition.to_state);
        state_updater.send(state_transition.to_state);
    }
}
