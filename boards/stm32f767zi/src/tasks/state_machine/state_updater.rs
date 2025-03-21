use crate::tasks::can::receive::INCOMING_STATE_TRANSITIONS;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use hyped_state_machine::states::State;

use {defmt_rtt as _, panic_probe as _};

/// Task that updates the current state of the system by receiving state transitions from the CAN.
#[embassy_executor::task]
pub async fn state_updater(state_updater: Sender<'static, CriticalSectionRawMutex, State, 1>) {
    let incoming_state_transitions = INCOMING_STATE_TRANSITIONS.receiver();

    loop {
        let state_transition = incoming_state_transitions.receive().await;
        defmt::info!("Changing state: {:?}", state_transition.to_state);
        state_updater.send(state_transition.to_state);
    }
}
