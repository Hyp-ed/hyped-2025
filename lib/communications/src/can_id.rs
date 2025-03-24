use super::{boards::Board, data::CanDataType, identifier::MessageIdentifier};

#[derive(Debug, PartialEq, Clone)]
pub struct CanId {
    pub board: Board,
    pub message_type: CanDataType,
    pub message_identifier: MessageIdentifier,
}

impl CanId {
    pub fn new(
        board: Board,
        message_type: CanDataType,
        message_identifier: MessageIdentifier,
    ) -> Self {
        CanId {
            board,
            message_type,
            message_identifier,
        }
    }
}

impl From<CanId> for u32 {
    fn from(val: CanId) -> Self {
        let board: u8 = val.board.into();
        let message_type: u8 = val.message_type.into();
        let message_identifier: u16 = val.message_identifier.into();

        // Format: board message_type message_identifier
        ((board as u32) << 24) | ((message_type as u32) << 16) | (message_identifier as u32)
    }
}

impl From<u32> for CanId {
    fn from(id: u32) -> Self {
        let board: Board = (((id >> 24) & 0xFF) as u8).into();
        let message_type: CanDataType = (((id >> 16) & 0xFF) as u8).into();
        let message_identifier: MessageIdentifier = (((id) & 0xFFFF) as u16).into();

        CanId {
            board,
            message_type,
            message_identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let can_id = CanId::new(
            Board::Test,
            CanDataType::State,
            MessageIdentifier::StateTransitionCommand,
        );
        let id: u32 = can_id.clone().into();

        assert_eq!(can_id, CanId::from(id));
        assert_eq!(
            CanId::from(id).message_identifier,
            MessageIdentifier::StateTransitionCommand
        );
    }
}
