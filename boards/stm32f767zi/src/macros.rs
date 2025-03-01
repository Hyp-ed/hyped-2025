#[macro_export]
macro_rules! oh_god_oh_fuck {
    ($can_sender:ident, $board:ident) => {
        let can_message =
            CanMessage::StateTransition(StateTransition::new($board, State::EmergencyBrake));
        $can_sender.send(can_message).await;
    };
}

#[macro_export]
macro_rules! request_transition {
    ($state:expr, $can_sender:ident, $board:ident) => {
        let can_message = CanMessage::StateTransitionRequest(StateTransition::new($board, $state));
        $can_sender.send(can_message).await;
    };
}
