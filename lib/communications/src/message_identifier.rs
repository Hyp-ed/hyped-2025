use hyped_core::config::MeasurementId;

#[derive(Debug, PartialEq, Clone)]
pub enum MessageIdentifier {
    Measurement(MeasurementId),
    StateTransitionCommand,
    StateTransitionRequest,
    Heartbeat,
    Emergency,
}

impl From<MessageIdentifier> for u16 {
    fn from(val: MessageIdentifier) -> Self {
        match val {
            MessageIdentifier::Measurement(measurement_id) => measurement_id.into(),
            MessageIdentifier::Emergency => 0xFC,
            MessageIdentifier::Heartbeat => 0xFD,
            MessageIdentifier::StateTransitionRequest => 0xFE,
            MessageIdentifier::StateTransitionCommand => 0xFF,
        }
    }
}

impl From<u16> for MessageIdentifier {
    fn from(id: u16) -> Self {
        match id {
            0xFF => MessageIdentifier::StateTransitionCommand,
            0xFE => MessageIdentifier::StateTransitionRequest,
            0xFD => MessageIdentifier::Heartbeat,
            0xFC => MessageIdentifier::Emergency,
            _ => MessageIdentifier::Measurement(id.into()),
        }
    }
}
