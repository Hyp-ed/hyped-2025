use crate::can_open_message::{config_messages, messages, CanOpenMessage};
use hyped_can::{CanError, HypedCan, HypedCanFrame};

/// All types of messages that can be sent to the motor controller
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

// TODOLater consider adding a ReceivedMessagesEnum so we can decide what we do depending on the message we receive

/// Convert a CanOpenMessage to a HypedCanFrame
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

/// Convert a HypedCanFrame to a CanOpenMessage
impl From<HypedCanFrame> for CanOpenMessage {
    fn from(frame: HypedCanFrame) -> Self {
        CanOpenMessage {
            id: frame.can_id,
            command: frame.data[0],
            index: u16::from(frame.data[1]) | (u16::from(frame.data[2]) << 8),
            sub_index: frame.data[3],
            data: u32::from(frame.data[4])
                | (u32::from(frame.data[5]) << 8)
                | (u32::from(frame.data[6]) << 16)
                | (u32::from(frame.data[7]) << 24),
        }
    }
}

/// Convert a MessagesEnum to a CanOpenMessage
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

/// A wrapper around a HypedCan that turns a CanOpenMessage into a HypedCanFrame and sends it over the HypedCan
/// Also reads a HypedCanFrame and turns it into a CanOpenMessage
pub struct CanOpen<T: HypedCan> {
    can: T,
}

impl<T: HypedCan> CanOpen<T> {
    pub fn new(can: T) -> Self {
        CanOpen { can }
    }

    /// Send a message to the motor controller
    pub fn send_message(&mut self, message: MessagesEnum) -> Result<(), CanError> {
        let frame: HypedCanFrame = CanOpenMessage::from(message).into();
        self.can.write_frame(&frame)
    }

    /// Read a message from the motor controller and return it
    pub fn read_message(&mut self) -> Result<CanOpenMessage, CanError> {
        let envelope = self.can.read_frame()?;
        let frame = envelope.frame;
        let message = CanOpenMessage::from(frame);
        Ok(message)
    }
}
