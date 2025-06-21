use hyped_core::config::MeasurementId;

#[derive(Debug, PartialEq, Clone)]
pub enum MessageIdentifier {
    Measurement(MeasurementId),
    StateTransitionCommand,
    StateTransitionRequest,
    Heartbeat,
    Emergency,
}

// 12 bits
const MAX_MESSAGE_IDENTIFIER: u16 = 0xFFF;

const STATE_TRANSITION_COMMAND_ID: u16 = MAX_MESSAGE_IDENTIFIER - 1;
const STATE_TRANSITION_REQUEST_ID: u16 = MAX_MESSAGE_IDENTIFIER - 2;
const HEARTBEAT_ID: u16 = MAX_MESSAGE_IDENTIFIER - 3;
const EMERGENCY_ID: u16 = MAX_MESSAGE_IDENTIFIER - 4;

impl From<MessageIdentifier> for u16 {
    fn from(val: MessageIdentifier) -> Self {
        match val {
            MessageIdentifier::Measurement(measurement_id) => measurement_id.into(),
            MessageIdentifier::Emergency => EMERGENCY_ID,
            MessageIdentifier::Heartbeat => HEARTBEAT_ID,
            MessageIdentifier::StateTransitionRequest => STATE_TRANSITION_REQUEST_ID,
            MessageIdentifier::StateTransitionCommand => STATE_TRANSITION_COMMAND_ID,
        }
    }
}

impl TryFrom<u16> for MessageIdentifier {
    type Error = &'static str;

    fn try_from(id: u16) -> Result<Self, Self::Error> {
        match id {
            STATE_TRANSITION_COMMAND_ID => Ok(MessageIdentifier::StateTransitionCommand),
            STATE_TRANSITION_REQUEST_ID => Ok(MessageIdentifier::StateTransitionRequest),
            HEARTBEAT_ID => Ok(MessageIdentifier::Heartbeat),
            EMERGENCY_ID => Ok(MessageIdentifier::Emergency),
            _ => match MeasurementId::try_from(id) {
                Ok(measurement_id) => Ok(MessageIdentifier::Measurement(measurement_id)),
                Err(e) => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyped_core::config::MeasurementId;

    #[test]
    fn test_message_identifier_state_transition_command() {
        let message_identifier = MessageIdentifier::StateTransitionCommand;
        let encoded_message_identifier: u16 = message_identifier.clone().into();

        let decoded_message_identifier = MessageIdentifier::try_from(encoded_message_identifier)
            .expect("Failed to decode message identifier");
        assert_eq!(message_identifier, decoded_message_identifier);
    }

    #[test]
    fn test_message_identifier_state_transition_request() {
        let message_identifier = MessageIdentifier::StateTransitionRequest;
        let encoded_message_identifier: u16 = message_identifier.clone().into();

        let decoded_message_identifier = MessageIdentifier::try_from(encoded_message_identifier)
            .expect("Failed to decode message identifier");
        assert_eq!(message_identifier, decoded_message_identifier);
    }

    #[test]
    fn test_message_identifier_heartbeat() {
        let message_identifier = MessageIdentifier::Heartbeat;
        let encoded_message_identifier: u16 = message_identifier.clone().into();

        let decoded_message_identifier = MessageIdentifier::try_from(encoded_message_identifier)
            .expect("Failed to decode message identifier");
        assert_eq!(message_identifier, decoded_message_identifier);
    }

    #[test]
    fn test_message_identifier_measurement() {
        let message_identifier = MessageIdentifier::Measurement(MeasurementId::Thermistor1);
        let encoded_message_identifier: u16 = message_identifier.clone().into();

        let decoded_message_identifier = MessageIdentifier::try_from(encoded_message_identifier)
            .expect("Failed to decode message identifier");
        assert_eq!(message_identifier, decoded_message_identifier);
    }

    #[test]
    fn test_invalid_message_identifier() {
        assert!(MessageIdentifier::try_from(0xABCD).is_err());
    }
}
