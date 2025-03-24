use super::measurements::MeasurementId;

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
            MessageIdentifier::Heartbeat => 0xFD,
            MessageIdentifier::StateTransitionRequest => 0xFE,
            MessageIdentifier::StateTransitionCommand => 0xFF,
            MessageIdentifier::Emergency => 0xFC,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_identifier_into() {
        let measurement_id = MeasurementId::Acceleration;
        let message_identifier = MessageIdentifier::Measurement(measurement_id);
        let message_identifier: u16 = message_identifier.into();
        assert_eq!(message_identifier, 0x00);

        let message_identifier = MessageIdentifier::StateTransitionCommand;
        let message_identifier: u16 = message_identifier.into();
        assert_eq!(message_identifier, 0xFF);
    }

    #[test]
    fn test_message_identifier_from() {
        let message_identifier = MessageIdentifier::Measurement(MeasurementId::Acceleration);
        assert_eq!(MessageIdentifier::from(0x00), message_identifier);

        let message_identifier = MessageIdentifier::StateTransitionCommand;
        assert_eq!(MessageIdentifier::from(0xFF), message_identifier);
    }
}
