#[macro_export]
macro_rules! emergency {
    () => {
        let can_sender = CAN_SEND.sender();
        let can_message =
            CanMessage::StateTransition(StateTransition::new(BOARD, State::EmergencyBrake));
        can_sender.send(can_message).await;
    };
    ($board:ident) => {
        let can_sender = CAN_SEND.sender();
        let can_message =
            CanMessage::StateTransition(StateTransition::new($board, State::EmergencyBrake));
        can_sender.send(can_message).await;
    };
}

#[macro_export]
macro_rules! request_transition {
    ($state:expr) => {
        let can_sender = CAN_SEND.sender();
        let can_message = CanMessage::StateTransitionRequest(StateTransition::new(BOARD, $state));
        can_sender.send(can_message).await;
    };
}
