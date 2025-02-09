use crate::can_open_message::{config_messages, messages, CanOpenMessage};
use hyped_can::{CanError, HypedCan, HypedCanFrame};

pub enum MessagesEnum {
    TestStepperEnable,
    TestModeCommand,
    EnterStopState,
    EnterPreoperationalState,
    EnterOperationalState,
    SetFrequency(u32),
    Shutdown,
    SwitchOn,
    StartDrive,
    QuickStop,
}

impl From<CanOpenMessage> for HypedCanFrame {
    fn from(msg: CanOpenMessage) -> Self {
        let mut data: [u8; 8] = [0; 8];

        data[0] = msg.command;
        data[1] = (msg.index & 0xFF) as u8;
        data[2] = ((msg.index >> 8) & 0xFF) as u8;
        data[3] = msg.sub_index;
        data[4] = (msg.data & 0xFF) as u8;
        data[5] = ((msg.data >> 8) & 0xFF) as u8;
        data[6] = ((msg.data >> 16) & 0xFF) as u8;
        data[7] = ((msg.data >> 24) & 0xFF) as u8;

        HypedCanFrame {
            can_id: msg.id,
            data,
        }
    }
}

impl From<MessagesEnum> for CanOpenMessage {
    fn from(message: MessagesEnum) -> Self {
        match message {
            MessagesEnum::TestStepperEnable => config_messages::TEST_STEPPER_ENABLE,
            MessagesEnum::TestModeCommand => config_messages::TEST_MODE_COMMAND,
            MessagesEnum::EnterStopState => messages::ENTER_STOP_STATE,
            MessagesEnum::EnterPreoperationalState => messages::ENTER_PREOPERATIONAL_STATE,
            MessagesEnum::EnterOperationalState => messages::ENTER_OPERATIONAL_STATE,
            MessagesEnum::SetFrequency(f) => CanOpenMessage {
                id: messages::SET_FREQUENCY.id,
                command: messages::SET_FREQUENCY.command,
                index: messages::SET_FREQUENCY.index,
                sub_index: messages::SET_FREQUENCY.sub_index,
                data: f,
            },
            MessagesEnum::Shutdown => messages::SHUTDOWN,
            MessagesEnum::SwitchOn => messages::SWITCH_ON,
            MessagesEnum::StartDrive => messages::START_DRIVE,
            MessagesEnum::QuickStop => messages::QUICK_STOP,
        }
    }
}

pub struct CanOpen<T: HypedCan> {
    can: T,
}

impl<T: HypedCan> CanOpen<T> {
    pub fn new(can: T) -> Self {
        CanOpen { can }
    }

    pub fn send_message(&mut self, message: MessagesEnum) -> Result<(), CanError> {
        let frame: HypedCanFrame = CanOpenMessage::from(message).into();
        self.can.write_frame(&frame)
    }
}
