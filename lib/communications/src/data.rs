use crate::emergency::Reason;

use super::boards::Board;

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum CanData {
    Bool(bool),
    TwoU16([u16; 2]),
    F32(f32),
    State(u8),
    U32(u32),
    Heartbeat(Board),
    Emergency(Reason),
}

impl From<CanData> for u8 {
    /// Gets the index of the CanData enum
    fn from(val: CanData) -> Self {
        match val {
            CanData::Bool(_) => 0,
            CanData::TwoU16(_) => 1,
            CanData::F32(_) => 2,
            CanData::State(_) => 3,
            CanData::U32(_) => 4,
            CanData::Heartbeat(_) => 5,
            CanData::Emergency(_) => 6,
        }
    }
}

impl From<u8> for CanData {
    /// Gets the CanData enum from the index
    fn from(index: u8) -> Self {
        match index {
            0 => CanData::Bool(false),
            1 => CanData::TwoU16([0, 0]),
            2 => CanData::F32(0.0),
            3 => CanData::State(0),
            4 => CanData::U32(0),
            5 => CanData::Heartbeat(Board::Test),
            6 => CanData::Emergency(Reason::Unknown),
            _ => panic!("Invalid CanData index"),
        }
    }
}

impl From<CanData> for [u8; 8] {
    fn from(val: CanData) -> Self {
        match val {
            CanData::Bool(b) => {
                let mut data: [u8; 8] = [0; 8];
                data[0] = val.into();
                data[1] = b as u8;
                data
            }
            CanData::TwoU16(u16s) => {
                let mut data: [u8; 8] = [0; 8];
                data[0] = val.into();
                let u16_bytes: [u8; 2] = u16s[0].to_le_bytes();
                data[1..3].copy_from_slice(&u16_bytes);

                let u16_bytes: [u8; 2] = u16s[1].to_le_bytes();
                data[3..5].copy_from_slice(&u16_bytes);

                data
            }
            CanData::F32(f) => {
                let mut data: [u8; 8] = [0; 8];
                data[0] = val.into();
                let f32_bytes: [u8; 4] = f.to_le_bytes();
                data[1..5].copy_from_slice(&f32_bytes);
                data
            }
            CanData::State(s) => {
                let mut data: [u8; 8] = [0; 8];
                data[0] = val.into();
                data[1] = s;
                data
            }
            CanData::U32(u) => {
                let mut data: [u8; 8] = [0; 8];
                data[0] = val.into();
                let u32_bytes: [u8; 4] = u.to_le_bytes();
                data[1..5].copy_from_slice(&u32_bytes);
                data
            }
            CanData::Heartbeat(board) => {
                let mut data: [u8; 8] = [0; 8];
                data[0] = val.into();
                data[1] = board.into();
                data
            }
            CanData::Emergency(reason) => {
                let mut data: [u8; 8] = [0; 8];
                data[0] = val.into();
                data[1] = reason.into();
                data
            }
        }
    }
}

impl From<[u8; 8]> for CanData {
    fn from(data: [u8; 8]) -> Self {
        let data_type: CanData = data[0].into();
        match data_type {
            CanData::Bool(_) => CanData::Bool(data[1] != 0),
            CanData::TwoU16(_) => {
                let mut u16_bytes: [u8; 2] = [0; 2];
                u16_bytes.copy_from_slice(&data[1..3]);
                let u16_1 = u16::from_le_bytes(u16_bytes);

                u16_bytes.copy_from_slice(&data[3..5]);
                let u16_2 = u16::from_le_bytes(u16_bytes);

                CanData::TwoU16([u16_1, u16_2])
            }
            CanData::F32(_) => {
                let mut f32_bytes: [u8; 4] = [0; 4];
                f32_bytes.copy_from_slice(&data[1..5]);
                let f = f32::from_le_bytes(f32_bytes);
                CanData::F32(f)
            }
            CanData::State(_) => CanData::State(data[1]),
            CanData::U32(_) => {
                let mut u32_bytes: [u8; 4] = [0; 4];
                u32_bytes.copy_from_slice(&data[1..5]);
                let u = u32::from_le_bytes(u32_bytes);
                CanData::U32(u)
            }
            CanData::Heartbeat(_) => CanData::Heartbeat(data[1].into()),
            CanData::Emergency(_) => CanData::Emergency(data[1].into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum CanDataType {
    Bool,
    TwoU16,
    F32,
    State,
    U32,
    Heartbeat,
    Emergency,
}

impl From<CanDataType> for u8 {
    fn from(val: CanDataType) -> Self {
        match val {
            CanDataType::Bool => 0,
            CanDataType::TwoU16 => 1,
            CanDataType::F32 => 2,
            CanDataType::State => 3,
            CanDataType::U32 => 4,
            CanDataType::Heartbeat => 5,
            CanDataType::Emergency => 6,
        }
    }
}

impl From<u8> for CanDataType {
    fn from(index: u8) -> Self {
        match index {
            0 => CanDataType::Bool,
            1 => CanDataType::TwoU16,
            2 => CanDataType::F32,
            3 => CanDataType::State,
            4 => CanDataType::U32,
            5 => CanDataType::Heartbeat,
            6 => CanDataType::Emergency,
            _ => panic!("Invalid CanDataType index"),
        }
    }
}

impl From<CanData> for CanDataType {
    fn from(val: CanData) -> Self {
        match val {
            CanData::Bool(_) => CanDataType::Bool,
            CanData::TwoU16(_) => CanDataType::TwoU16,
            CanData::F32(_) => CanDataType::F32,
            CanData::State(_) => CanDataType::State,
            CanData::U32(_) => CanDataType::F32,
            CanData::Heartbeat(_) => CanDataType::Heartbeat,
            CanData::Emergency(_) => CanDataType::Emergency,
        }
    }
}

impl From<CanDataType> for CanData {
    fn from(data_type: CanDataType) -> Self {
        match data_type {
            CanDataType::Bool => CanData::Bool(false),
            CanDataType::TwoU16 => CanData::TwoU16([0, 0]),
            CanDataType::F32 => CanData::F32(0.0),
            CanDataType::State => CanData::State(0),
            CanDataType::U32 => CanData::U32(0),
            CanDataType::Heartbeat => CanData::Heartbeat(Board::Test),
            CanDataType::Emergency => CanData::Emergency(Reason::Unknown),
        }
    }
}
