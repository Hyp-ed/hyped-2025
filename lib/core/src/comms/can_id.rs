use super::{boards::Board, data::CanDataType, messages::MessageIdentifier};

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

impl Into<u32> for CanId {
    fn into(self) -> u32 {
        let board: u8 = self.board.into();
        let message_type: u8 = self.message_type.into();
        let message_identifier: u16 = self.message_identifier.into();

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
