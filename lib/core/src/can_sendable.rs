pub const CAN_MSG_TYPE_BOOL: u8 = 0;
pub const CAN_MSG_TYPE_F32 : u8 = 1;
pub const CAN_MSG_TYPE_U32 : u8 = 2;

pub trait CanSendable { 
    fn encode_to_can(&self, board_id:u8) -> [u8; 8];
}

pub fn build_can_header(board_id:u8, msg_type:u8) -> u8 {
    // 4 bits for each of board_id, msg_type
    (board_id << 4) | msg_type

}
    
impl CanSendable for bool {
    fn encode_to_can(&self, board_id:u8) -> [u8; 8] {
        let mut data: [u8;8] = [0;8];
        data[0] = build_can_header(board_id, CAN_MSG_TYPE_BOOL);
        data[1] = *self as u8; 
        return data;
    }
}

impl CanSendable for [u16; 2] {
    fn encode_to_can(&self, board_id:u8) -> [u8; 8] {
        let mut data: [u8;8] = [0;8];
        data[0] = build_can_header(board_id, CAN_MSG_TYPE_U32);

        let u16_bytes: [u8; 2] = self[0].to_le_bytes();
        data[1..3].copy_from_slice(&u16_bytes);

        let u16_bytes: [u8; 2] = self[1].to_le_bytes();
        data[3..5].copy_from_slice(&u16_bytes);

        return data;
    }
    
}

impl CanSendable for f32 {
    fn encode_to_can(&self, board_id:u8) -> [u8; 8] {
        let mut data: [u8;8] = [0;8];
        data[0] = build_can_header(board_id, CAN_MSG_TYPE_F32);

        let f32_bytes: [u8; 4] = self.to_le_bytes();
        data[1..5].copy_from_slice(&f32_bytes);
        return data;
    }
}