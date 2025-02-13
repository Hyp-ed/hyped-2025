/// Basic structure for a CanOpen Message
pub struct CanOpenMessage {
    pub id: u32,
    pub index: u16,
    pub sub_index: u8,
    pub command: u8,
    pub data: u32,
}

pub mod config_messages {
    use super::CanOpenMessage;

    pub const TEST_STEPPER_ENABLE: CanOpenMessage = CanOpenMessage {
        id: 0x601,
        command: 0x2B,
        index: 0x6040,
        sub_index: 0x09,
        data: 0x00000001,
    };

    pub const TEST_MODE_COMMAND: CanOpenMessage = CanOpenMessage {
        id: 0x601,
        command: 0x2B,
        index: 0x2031,
        sub_index: 0x00,
        data: 0x00000060,
    };
}

pub mod messages {
    use super::CanOpenMessage;

    pub const ENTER_STOP_STATE: CanOpenMessage = CanOpenMessage {
        id: 0x000,
        command: 0x02,
        index: 0x0000,
        sub_index: 0x00,
        data: 0x00000000,
    };

    pub const ENTER_PREOPERATIONAL_STATE: CanOpenMessage = CanOpenMessage {
        id: 0x000,
        command: 0x03,
        index: 0x0000,
        sub_index: 0x00,
        data: 0x00000000,
    };

    pub const ENTER_OPERATIONAL_STATE: CanOpenMessage = CanOpenMessage {
        id: 0x000,
        command: 0x01,
        index: 0x0000,
        sub_index: 0x00,
        data: 0x00000000,
    };

    // Data will be overwritten at runtime depending on the frequency desired
    pub const SET_FREQUENCY: CanOpenMessage = CanOpenMessage {
        id: 0x601,
        command: 0x2B,
        index: 0x2040,
        sub_index: 0x04,
        data: 0x00000000,
    };

    pub const SHUTDOWN: CanOpenMessage = CanOpenMessage {
        id: 0x601,
        command: 0x2B,
        index: 0x6040,
        sub_index: 0x00,
        data: 0x00000006,
    };

    pub const SWITCH_ON: CanOpenMessage = CanOpenMessage {
        id: 0x601,
        command: 0x2B,
        index: 0x6040,
        sub_index: 0x00,
        data: 0x00000007,
    };

    pub const START_DRIVE: CanOpenMessage = CanOpenMessage {
        id: 0x601,
        command: 0x2B,
        index: 0x6040,
        sub_index: 0x00,
        data: 0x0000000F,
    };

    pub const QUICK_STOP: CanOpenMessage = CanOpenMessage {
        id: 0x601,
        command: 0x2B,
        index: 0x6040,
        sub_index: 0x00,
        data: 0x00000002,
    };
}
