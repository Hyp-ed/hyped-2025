use super::{boards::Board, data::CanDataType, message_identifier::MessageIdentifier};

#[derive(Debug, PartialEq, Clone)]
pub struct CanId {
    pub priority: bool,
    pub board: Board,
    pub message_data_type: CanDataType,
    pub message_identifier: MessageIdentifier,
}

impl CanId {
    /// Creates a new `CanId` with the given parameters. Defaults to standard priority.
    pub fn new(
        board: Board,
        message_type: CanDataType,
        message_identifier: MessageIdentifier,
    ) -> Self {
        CanId {
            priority: false,
            board,
            message_data_type: message_type,
            message_identifier,
        }
    }

    /// Creates a new `CanId` with the given parameters. Sets the priority to high.
    pub fn new_high_priority(
        board: Board,
        message_type: CanDataType,
        message_identifier: MessageIdentifier,
    ) -> Self {
        CanId {
            priority: true,
            board,
            message_data_type: message_type,
            message_identifier,
        }
    }
}

impl From<CanId> for u32 {
    fn from(val: CanId) -> Self {
        let priority: u32 = if val.priority { 1 } else { 0 };
        let board: u32 = u8::from(val.board) as u32;
        let message_type: u32 = u8::from(val.message_data_type) as u32;
        let message_identifier: u32 = u16::from(val.message_identifier) as u32;

        // Make sure that measurement_identifier is 13 bits
        assert!(message_identifier < (1 << 13));

        // Format: priority (1 bit) | message_type (8 bits) | message_identifier (12 bits) | board (8 bits) = 29 bits
        (priority << 28) | (message_type << 20) | (message_identifier << 8) | board
    }
}

macro_rules! extract_bits {
    ($id:expr, $start:expr, $end:expr) => {
        ($id >> $start) & ((1 << ($end - $start)) - 1)
    };
}

impl From<u32> for CanId {
    fn from(id: u32) -> Self {
        let priority = extract_bits!(id, 28, 29) == 1;
        let message_type = CanDataType::try_from(extract_bits!(id, 20, 28) as u8)
            .expect("Failed to decode message type");
        let message_identifier = MessageIdentifier::try_from(extract_bits!(id, 8, 20) as u16)
            .expect("Failed to decode message identifier");
        let board = Board::try_from(extract_bits!(id, 0, 8) as u8).expect("Failed to decode board");

        CanId {
            priority,
            board,
            message_data_type: message_type,
            message_identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let can_id = CanId::new_high_priority(
            Board::Test,
            CanDataType::State,
            MessageIdentifier::StateTransitionCommand,
        );
        let encoded_can_id: u32 = can_id.clone().into();

        assert_eq!(can_id, CanId::from(encoded_can_id));
    }

    #[test]
    fn it_works_with_low_priority() {
        let can_id = CanId::new(
            Board::Test,
            CanDataType::State,
            MessageIdentifier::StateTransitionCommand,
        );
        let encoded_can_id: u32 = can_id.clone().into();

        assert_eq!(can_id, CanId::from(encoded_can_id));
    }
}
