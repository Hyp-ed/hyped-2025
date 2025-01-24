use defmt::error;

pub const CAN_MSG_TYPE_BOOL: u8 = 0;
pub const CAN_MSG_TYPE_F32 : u8 = 1;
pub const CAN_MSG_TYPE_U32 : u8 = 2;
pub const CAN_MSG_TYPE_POSDELTA : u8 = 3;

pub enum CanMsgType {
    Bool,
    F32,
    U32,
    PosDelta,
}

pub fn can_msg_type_from_u8(msg_type: &[u8; 8]) -> CanMsgType {
    match msg_type[0] & 0x0F {
        CAN_MSG_TYPE_BOOL => CanMsgType::Bool,
        CAN_MSG_TYPE_F32 => CanMsgType::F32,
        CAN_MSG_TYPE_U32 => CanMsgType::U32,
        CAN_MSG_TYPE_POSDELTA => CanMsgType::PosDelta,
        _ => {
            error!("Unknown CAN message type: {}", msg_type[0] & 0x0F);
            panic!();
        },
    }
}

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

pub struct PositionDelta {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for PositionDelta {
    fn default() -> Self {
        PositionDelta {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl PositionDelta {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        PositionDelta {
            x,
            y,
            z,
        }
    }

    pub fn encode_to_can(&self, board_id:u8) -> [[u8; 8]; 3] {
        let mut x = self.x.encode_to_can(board_id);
        let mut y = self.y.encode_to_can(board_id);        
        let mut z = self.z.encode_to_can(board_id);
        x[0] = build_can_header(board_id, CAN_MSG_TYPE_POSDELTA);
        y[0] = build_can_header(board_id, CAN_MSG_TYPE_POSDELTA);
        z[0] = build_can_header(board_id, CAN_MSG_TYPE_POSDELTA);
        
        x[5] = 0;
        y[5] = 1;
        z[5] = 2;
        [x, y, z]
    }
}
