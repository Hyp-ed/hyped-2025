use core::str::FromStr;
use heapless::String;

#[derive(PartialEq, Debug, defmt::Format, Clone, Copy)]
pub enum State {
    Idle,
    Calibrate,
    Precharge,
    ReadyForLevitation,
    BeginLevitation,
    Ready,
    Accelerate,
    Brake,
    StopLevitation,
    Stopped,
    Emergency,
}

impl From<State> for u8 {
    fn from(val: State) -> Self {
        match val {
            State::Idle => 0x00,
            State::Calibrate => 0x01,
            State::Precharge => 0x02,
            State::ReadyForLevitation => 0x03,
            State::BeginLevitation => 0x04,
            State::Ready => 0x05,
            State::Accelerate => 0x06,
            State::Brake => 0x07,
            State::StopLevitation => 0x08,
            State::Stopped => 0x09,
            State::Emergency => 0x0A,
        }
    }
}

impl TryFrom<u8> for State {
    type Error = ();

    /// Convert a u8 into a State. Returns an error if the u8 is not a valid State.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(State::Idle),
            0x01 => Ok(State::Calibrate),
            0x02 => Ok(State::Precharge),
            0x03 => Ok(State::ReadyForLevitation),
            0x04 => Ok(State::BeginLevitation),
            0x05 => Ok(State::Ready),
            0x06 => Ok(State::Accelerate),
            0x07 => Ok(State::Brake),
            0x08 => Ok(State::StopLevitation),
            0x09 => Ok(State::Stopped),
            0x0A => Ok(State::Emergency),
            _ => Err(()),
        }
    }
}

impl State {
    pub fn to_string(&self) -> String<20> {
        match self {
            State::Idle => String::<20>::from_str("idle").unwrap(),
            State::Calibrate => String::<20>::from_str("calibrate").unwrap(),
            State::Precharge => String::<20>::from_str("precharge").unwrap(),
            State::ReadyForLevitation => String::<20>::from_str("ready_for_levitation").unwrap(),
            State::BeginLevitation => String::<20>::from_str("begin_levitation").unwrap(),
            State::Ready => String::<20>::from_str("ready").unwrap(),
            State::Accelerate => String::<20>::from_str("accelerate").unwrap(),
            State::Brake => String::<20>::from_str("brake").unwrap(),
            State::StopLevitation => String::<20>::from_str("stop_levitation").unwrap(),
            State::Stopped => String::<20>::from_str("stopped").unwrap(),
            State::Emergency => String::<20>::from_str("emergency").unwrap(),
        }
    }
}

impl Into<String<20>> for State {
    fn into(self) -> String<20> {
        self.to_string()
    }
}

impl TryFrom<&str> for State {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "idle" => Ok(State::Idle),
            "calibrate" => Ok(State::Calibrate),
            "precharge" => Ok(State::Precharge),
            "ready_for_levitation" => Ok(State::ReadyForLevitation),
            "begin_levitation" => Ok(State::BeginLevitation),
            "ready" => Ok(State::Ready),
            "accelerate" => Ok(State::Accelerate),
            "brake" => Ok(State::Brake),
            "stop_levitation" => Ok(State::StopLevitation),
            "stopped" => Ok(State::Stopped),
            "emergency" => Ok(State::Emergency),
            _ => Err(()),
        }
    }
}
