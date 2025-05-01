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

impl TryFrom<u16> for MessageIdentifier {
    type Error = &'static str;

    fn try_from(id: u16) -> Result<Self, Self::Error> {
        match id {
            0xFF => Ok(MessageIdentifier::StateTransitionCommand),
            0xFE => Ok(MessageIdentifier::StateTransitionRequest),
            0xFD => Ok(MessageIdentifier::Heartbeat),
            0xFC => Ok(MessageIdentifier::Emergency),
            _ => match id.try_into() {
                Ok(measurement_id) => Ok(measurement_id),
                Err(_) => Err("Failed to parse MessageIdentifier"),
            },
        }
    }
}
