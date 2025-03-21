use crate::tasks::can::{receive::INCOMING_STATE_TRANSITION_REQUESTS, send::CAN_SEND};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_communications::{
    boards::Board, messages::CanMessage, state_transition::StateTransition,
};
use hyped_state_machine::state_machine::StateMachine;
use hyped_state_machine::states::State;
use {defmt_rtt as _, panic_probe as _};

/// Handles the state machine logic by receiving state transition requests and sending new states.
#[embassy_executor::task]
pub async fn state_machine(
    board: Board,
    state_sender: Sender<'static, CriticalSectionRawMutex, State, 1>,
) {
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

                // Update this board's state
                state_sender.send(state);

                // Send the new state to the CAN bus
                let can_message = CanMessage::StateTransition(StateTransition::new(board, state));
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

        Timer::after(Duration::from_millis(10)).await;
    }
}
