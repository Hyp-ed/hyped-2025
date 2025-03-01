#[macro_export]
macro_rules! trigger_emergency {
    ($can_sender:ident, $board:ident) => {
        let can_message =
            CanMessage::StateTransition(StateTransition::new($board, State::EmergencyBrake));
        $can_sender.send(can_message).await;
    };
}
