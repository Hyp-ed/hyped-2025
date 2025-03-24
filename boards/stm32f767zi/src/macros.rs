/// Sends an emergency message over CAN with the given reason.
/// Will cause all boards to transition to the Emergency state.
#[macro_export]
macro_rules! emergency {
    () => {
        let can_sender = CAN_SEND.sender();
        let can_message = CanMessage::Emergency(BOARD, Reason::Unknown);
        can_sender.send(can_message).await;
    };
    ($board:ident) => {
        let can_sender = CAN_SEND.sender();
        let can_message = CanMessage::Emergency($board, Reason::Unknown);
        can_sender.send(can_message).await;
    };
}

/// Sends a state transition request to the state machine over CAN.
#[macro_export]
macro_rules! request_transition {
    ($state:expr) => {
        let can_sender = CAN_SEND.sender();
        let can_message =
            CanMessage::StateTransitionRequest(StateTransitionRequest::new(BOARD, $state));
        can_sender.send(can_message).await;
    };
}
