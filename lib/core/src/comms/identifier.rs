use super::measurements::MeasurementId;

#[derive(Debug, PartialEq, Clone)]
pub enum MessageIdentifier {
    Measurement(MeasurementId),
    StateTransition,
    StateTransitionRequest,
    Heartbeat,
}

impl From<MessageIdentifier> for u16 {
    fn from(val: MessageIdentifier) -> Self {
        match val {
            MessageIdentifier::Measurement(measurement_id) => measurement_id.into(),
            MessageIdentifier::Heartbeat => 0xFD,
            MessageIdentifier::StateTransitionRequest => 0xFE,
            MessageIdentifier::StateTransition => 0xFF,
        }
    }
}

impl From<u16> for MessageIdentifier {
    fn from(id: u16) -> Self {
        match id {
            0xFF => MessageIdentifier::StateTransition,
            0xFE => MessageIdentifier::StateTransitionRequest,
            0xFD => MessageIdentifier::Heartbeat,
            _ => MessageIdentifier::Measurement(id.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_identifier_into() {
        let measurement_id = MeasurementId::Temperature;
        let message_identifier = MessageIdentifier::Measurement(measurement_id);
        let message_identifier: u16 = message_identifier.into();
        assert_eq!(message_identifier, 0x00);

        let message_identifier = MessageIdentifier::StateTransition;
        let message_identifier: u16 = message_identifier.into();
        assert_eq!(message_identifier, 0xFF);
    }

    #[test]
    fn test_message_identifier_from() {
        let message_identifier = MessageIdentifier::Measurement(MeasurementId::Temperature);
        assert_eq!(MessageIdentifier::from(0x00), message_identifier);

        let message_identifier = MessageIdentifier::StateTransition;
        assert_eq!(MessageIdentifier::from(0xFF), message_identifier);
    }
}
