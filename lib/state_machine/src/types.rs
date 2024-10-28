use core::str::FromStr;
use heapless::String;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Idle,
    Calibrate,
    Precharge,
    ReadyForLevitation,
    BeginLevitation,
    Levitating,
    Ready,
    Accelerate,
    LimBrake,
    FrictionBrake,
    StopLevitation,
    Stopped,
    BatteryRecharge,
    CapacitorDischarge,
    FailureBrake,
    Failure,
    Safe,
    Shutdown,
}

impl State {
    pub fn to_string(&self) -> String<20> {
        match self {
            State::Idle => String::<20>::from_str("idle").unwrap(),
            State::Calibrate => String::<20>::from_str("calibrate").unwrap(),
            State::Precharge => String::<20>::from_str("precharge").unwrap(),
            State::ReadyForLevitation => String::<20>::from_str("ready_for_levitation").unwrap(),
            State::BeginLevitation => String::<20>::from_str("begin_levitation").unwrap(),
            State::Levitating => String::<20>::from_str("levitating").unwrap(),
            State::Ready => String::<20>::from_str("ready").unwrap(),
            State::Accelerate => String::<20>::from_str("accelerate").unwrap(),
            State::LimBrake => String::<20>::from_str("lim_brake").unwrap(),
            State::FrictionBrake => String::<20>::from_str("friction_brake").unwrap(),
            State::StopLevitation => String::<20>::from_str("stop_levitation").unwrap(),
            State::Stopped => String::<20>::from_str("stopped").unwrap(),
            State::BatteryRecharge => String::<20>::from_str("battery_recharge").unwrap(),
            State::CapacitorDischarge => String::<20>::from_str("capacitor_discharge").unwrap(),
            State::FailureBrake => String::<20>::from_str("failure_brake").unwrap(),
            State::Failure => String::<20>::from_str("failure").unwrap(),
            State::Safe => String::<20>::from_str("safe").unwrap(),
            State::Shutdown => String::<20>::from_str("shutdown").unwrap(),
        }
    }

    pub fn from_string(state: &str) -> Option<State> {
        match state {
            "idle" => Some(State::Idle),
            "calibrate" => Some(State::Calibrate),
            "precharge" => Some(State::Precharge),
            "ready_for_levitation" => Some(State::ReadyForLevitation),
            "begin_levitation" => Some(State::BeginLevitation),
            "levitating" => Some(State::Levitating),
            "ready" => Some(State::Ready),
            "accelerate" => Some(State::Accelerate),
            "lim_brake" => Some(State::LimBrake),
            "friction_brake" => Some(State::FrictionBrake),
            "stop_levitation" => Some(State::StopLevitation),
            "stopped" => Some(State::Stopped),
            "battery_recharge" => Some(State::BatteryRecharge),
            "capacitor_discharge" => Some(State::CapacitorDischarge),
            "failure_brake" => Some(State::FailureBrake),
            "failure" => Some(State::Failure),
            "safe" => Some(State::Safe),
            "shutdown" => Some(State::Shutdown),
            _ => None,
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct SourceAndTarget {
    pub(crate) source: State,
    pub(crate) target: State,
}
