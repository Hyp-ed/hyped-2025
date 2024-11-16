struct Messages {
    id: u16,
    command: u8,
    index: u16,
    subindex: u8,
    data: u32,
} 

const TEST_STEPPER_ENABLE: Message = Message {
    id: 0x601,
    command: 0x2B,
    index: 0x6040,
    subindex: 0x09,
    data: 0x00000001,
};

const TEST_MODE_COMMAND: Message = Message {
    id: 0x601,
    command: 0x2B,
    index: 0x2031,
    subindex: 0x00,
    data: 0x00000060,
};

const ENTER_STOP_STATE: Message = Message {
    id: 0x000,
    command: 0x02,
    index: 0x0000,
    subindex: 0x00,
    data: 0x00000000,
};

const ENTER_PREOPERATIONAL_STATE: Message = Message {
    id: 0x000,
    command: 0x03,
    index: 0x0000,
    subindex: 0x00,
    data: 0x00000000,
};

const ENTER_OPERATIONAL_STATE: Message = Message {
    id: 0x000,
    command: 0x01,
    index: 0x0000,
    subindex: 0x00,
    data: 0x00000000,
};

const SET_FREQUENCY: Message = Message {
    id: 0x601,
    command: 0x2B,
    index: 0x2040,
    subindex: 0x04,
    data: 0x00000000,
};

const SHUTDOWN: Message = Message {
    id: 0x601,
    command: 0x2B,
    index: 0x6040,
    subindex: 0x00,
    data: 0x00000006,
};

const SWITCH_ON: Message = Message {
    id: 0x601,
    command: 0x2B,
    index: 0x6040,
    subindex: 0x00,
    data: 0x00000007,
};

const START_DRIVE: Message = Message {
    id: 0x601,
    command: 0x2B,
    index: 0x6040,
    subindex: 0x00,
    data: 0x0000000F,
};

const QUICK_STOP: Message = Message {
    id: 0x601,
    command: 0x2B,
    index: 0x6040,
    subindex: 0x00,
    data: 0x00000002,
};