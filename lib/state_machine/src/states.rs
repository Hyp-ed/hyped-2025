use core::str::FromStr;
use heapless::String;

#[derive(PartialEq, Debug, defmt::Format, Clone, Copy)]
#[repr(u8)]
pub enum State {
    Idle = 0,
    Calibrate = 1,
    Precharge = 2,
    ReadyForLevitation = 3,
    BeginLevitation = 4,
    Ready = 5,
    Accelerate = 6,
    Brake = 7,
    StopLevitation = 8,
    Stopped = 9,
    Emergency = 10,
}

impl From<State> for u8 {
    fn from(v: State) -> Self {
        v as u8
    }
}

impl TryFrom<u8> for State {
    type Error = &'static str;

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
            _ => Err("Invalid state"),
        }
    }
}

impl From<State> for &str {
    fn from(val: State) -> Self {
        match val {
            State::Idle => "idle",
            State::Calibrate => "calibrate",
            State::Precharge => "precharge",
            State::ReadyForLevitation => "ready_for_levitation",
            State::BeginLevitation => "begin_levitation",
            State::Ready => "ready",
            State::Accelerate => "accelerate",
            State::Brake => "brake",
            State::StopLevitation => "stop_levitation",
            State::Stopped => "stopped",
            State::Emergency => "emergency",
        }
    }
}

impl FromStr for State {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
            _ => Err("Invalid state"),
        }
    }
}

impl From<State> for String<20> {
    fn from(val: State) -> Self {
        let mut s = String::new();
        s.push_str(val.into()).unwrap();
        s
    }
}
