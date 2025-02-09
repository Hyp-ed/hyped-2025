use defmt::error;

pub const CAN_MSG_TYPE_BOOL: u8 = 0;
pub const CAN_MSG_TYPE_F32: u8 = 1;
pub const CAN_MSG_TYPE_U32: u8 = 2;
pub const CAN_MSG_TYPE_POSDELTA: u8 = 3;

/// Checks if a CAN message is a valid message
pub fn is_valid_can_msg(msg: &[u8; 8]) -> bool {
    // we assume that board id is valid as ATM we dont know how many boards sending messages there are
    (msg[0] & 0x0F) < 4
}

/// Extracts the message type from a CAN message
pub fn can_msg_type_from_u8(msg: &[u8; 8]) -> CanMsgType {
    match msg[0] & 0x0F {
        CAN_MSG_TYPE_BOOL => CanMsgType::Bool,
        CAN_MSG_TYPE_F32 => CanMsgType::F32,
        CAN_MSG_TYPE_U32 => CanMsgType::U32,
        CAN_MSG_TYPE_POSDELTA => CanMsgType::PosDelta,
        _ => {
            error!("Unknown CAN message type: {}", msg[0] & 0x0F);
            panic!();
        }
    }
}

/// Extracts the board id from a CAN message
pub fn get_board_id(msg: &[u8; 8]) -> u8 {
    msg[0] >> 4
}

/// Builds a CAN header byte from a board id and message type
pub fn build_can_header(board_id: u8, msg_type: u8) -> u8 {
    (board_id << 4) | msg_type
}

pub enum CanMsgType {
    Bool,
    F32,
    U32,
    PosDelta,
}

pub trait CanSendable {
    fn encode_to_can(&self, board_id: u8) -> [u8; 8];
    fn can_decode(can_msg: &[u8; 8]) -> Self;
}

impl CanSendable for bool {
    fn encode_to_can(&self, board_id: u8) -> [u8; 8] {
        let mut data: [u8; 8] = [0; 8];
        data[0] = build_can_header(board_id, CAN_MSG_TYPE_BOOL);
        data[1] = *self as u8;
        data
    }

    fn can_decode(can_msg: &[u8; 8]) -> Self {
        can_msg[1] != 0
    }
}

impl CanSendable for [u16; 2] {
    fn encode_to_can(&self, board_id: u8) -> [u8; 8] {
        let mut data: [u8; 8] = [0; 8];
        data[0] = build_can_header(board_id, CAN_MSG_TYPE_U32);

        let u16_bytes: [u8; 2] = self[0].to_le_bytes();
        data[1..3].copy_from_slice(&u16_bytes);

        let u16_bytes: [u8; 2] = self[1].to_le_bytes();
        data[3..5].copy_from_slice(&u16_bytes);

        data
    }

    fn can_decode(can_msg: &[u8; 8]) -> Self {
        let mut u16_bytes: [u8; 2] = [0; 2];
        u16_bytes.copy_from_slice(&can_msg[1..3]);
        let u16_1 = u16::from_le_bytes(u16_bytes);

        u16_bytes.copy_from_slice(&can_msg[3..5]);
        let u16_2 = u16::from_le_bytes(u16_bytes);

        [u16_1, u16_2]
    }
}

impl CanSendable for f32 {
    fn encode_to_can(&self, board_id: u8) -> [u8; 8] {
        let mut data: [u8; 8] = [0; 8];
        data[0] = build_can_header(board_id, CAN_MSG_TYPE_F32);

        let f32_bytes: [u8; 4] = self.to_le_bytes();
        data[1..5].copy_from_slice(&f32_bytes);
        data
    }

    fn can_decode(can_msg: &[u8; 8]) -> Self {
        let mut f32_bytes: [u8; 4] = [0; 4];
        f32_bytes.copy_from_slice(&can_msg[1..5]);
        f32::from_le_bytes(f32_bytes)
    }
}

// Struct to hold f32 so they can be sent and decoded over CAN
pub struct PositionDelta {
    pub clock: Option<u8>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
}

impl PositionDelta {
    // sender side code
    pub fn new(clock: u8, x: f32, y: f32, z: f32) -> Self {
        PositionDelta {
            clock: Some(clock),
            x: Some(x),
            y: Some(y),
            z: Some(z),
        }
    }

    // converts a provided F32
    pub fn encode_to_can(&self, board_id: u8) -> [[u8; 8]; 3] {
        assert!(
            self.is_complete(),
            "Attempted to encode an incomplete PositionDelta"
        );

        let mut x = self.x.unwrap().encode_to_can(board_id);
        let mut y = self.y.unwrap().encode_to_can(board_id);
        let mut z = self.z.unwrap().encode_to_can(board_id);

        x[0] = build_can_header(board_id, CAN_MSG_TYPE_POSDELTA);
        y[0] = build_can_header(board_id, CAN_MSG_TYPE_POSDELTA);
        z[0] = build_can_header(board_id, CAN_MSG_TYPE_POSDELTA);

        x[5] = 0;
        y[5] = 1;
        z[5] = 2;

        let clock = self.clock.unwrap();
        x[6] = clock;
        y[6] = clock;
        z[6] = clock;
        [x, y, z]
    }
}

impl PositionDelta {
    // receiver code

    /// Create empty PositionDelta to fill
    pub fn new_empty() -> Self {
        PositionDelta {
            clock: None,
            x: None,
            y: None,
            z: None,
        }
    }

    /// Reset position delta to empty
    pub fn clear(&mut self) {
        self.clock = None;
        self.x = None;
        self.y = None;
        self.z = None;
    }

    /// Check we've received a completed PositionDelta, (all x,y,z all with the same clock value)
    pub fn is_complete(&self) -> bool {
        self.clock.is_some() && self.x.is_some() && self.y.is_some() && self.z.is_some()
    }

    /// Returns finished f32
    pub fn return_complete(&self) -> [f32; 3] {
        assert!(
            self.is_complete(),
            "Attempted to finalise an incomplete PositionDelta"
        );
        [self.x.unwrap(), self.y.unwrap(), self.z.unwrap()]
    }

    /// Attempt to decode a CAN PositionDelta value and add it to the current structure
    /// example implementation
    /// ```NoRun // cos doctests dont like no_std
    /// use hyped_core::can_sendable::*;
    /// 
    /// let mut pos_d = PositionDelta::new_empty();
    /// let mut err_cnt = 0;
    /// loop {
    ///     let next_val = [0; 8]; // get next CAN msg
    ///     if let Some(err) = pos_d.can_decode_step(next_val){
    ///         match err {
    ///             PositionDeltaDecodeError::PositionAlreadyReceived => { /* ... throw error and panic */ }
    ///             PositionDeltaDecodeError::ClockOutdated => { /* we dont need to do anything*/ }
    ///             PositionDeltaDecodeError::ClockInFuture => {
    ///                 err_cnt += 1;
    ///                 pos_d.clear();
    ///                 pos_d.can_decode_step(next_val); // will never error on first call after clear
    ///             }
    ///         }
    ///     }
    ///     if (err_cnt > 3) {
    ///         // ... emergency exit    
    ///     }
    ///     
    ///     if (pos_d.is_complete()){
    ///         let err_cnt = 0;
    ///         let vals = pos_d.return_complete();
    ///         // ... do something with complete values
    ///     }  
    /// }
    /// ```
    pub fn can_decode_step(&mut self, step: [u8; 8]) -> Option<PositionDeltaDecodeError> {
        let step_type = step[5];
        let step_clock = step[6];

        if self.clock.is_none() {
            self.clock = Some(step_clock);
        } else if self.clock.unwrap() != step_clock {
            return Some(
                if Self::is_clock_infuture(self.clock.unwrap(), step_clock) {
                    PositionDeltaDecodeError::ClockInFuture
                } else {
                    PositionDeltaDecodeError::ClockOutdated
                },
            );
        }

        let step_data = f32::can_decode(&step);
        match step_type {
            0 => {
                if self.x.is_none() {
                    self.x = Some(step_data);
                } else {
                    return Some(PositionDeltaDecodeError::PositionAlreadyReceived);
                }
            }
            1 => {
                if self.y.is_none() {
                    self.y = Some(step_data);
                } else {
                    return Some(PositionDeltaDecodeError::PositionAlreadyReceived);
                }
            }
            2 => {
                if self.z.is_none() {
                    self.z = Some(step_data);
                } else {
                    return Some(PositionDeltaDecodeError::PositionAlreadyReceived);
                }
            }
            _ => {
                error!("Unknown PositionDelta step type: {}", step_type);
                panic!();
            }
        }
        None
    }

    /// Checks if new clock is infront of the old
    pub fn is_clock_infuture(old: u8, new: u8) -> bool {
        let diff = new.wrapping_sub(old);
        diff < 128 // assume any diff > 128 is a wraparound
    }

    /// (synonym for `i.wrapping_add(1)` just used for sake of clarity)
    pub fn next_clock(old: u8) -> u8 {
        old.wrapping_add(1)
    }
}

/// All the possible failure reasons for failing to decode a position delta step
pub enum PositionDeltaDecodeError {
    /// The x y or z value for a given clock cycle has already been received.
    /// This error shouldn't ever be seen in normal operation, if it is that
    /// means that there is a **major issue** with the execution flow, either the
    /// CAN sender of the board is sending repeats or were are somehow an
    /// entire u8 out of sync (should be reason to stop the pod)
    PositionAlreadyReceived,
    /// The clock value we've received is in the future and so the current
    /// cycle should be discarded. The no. of discarded cycles should be
    /// recorded and if it reaches a given threshold (recommended 3~5) the
    /// pod should come to a stop as its current readings are too out of date
    ClockInFuture,
    /// Clock cycle of an old clock cycle has been received
    ClockOutdated,
}
