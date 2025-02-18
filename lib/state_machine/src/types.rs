use core::str::FromStr;
use heapless::String;

#[derive(PartialEq, Debug)]
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
    EmergencyBrake,
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
            State::EmergencyBrake => String::<20>::from_str("emergency_brake").unwrap(),
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
            "emergency_brake" => Some(State::EmergencyBrake),
            "safe" => Some(State::Safe),
            "shutdown" => Some(State::Shutdown),
            _ => None,
        }
    }

    pub fn transition(current_state: &State, to_state: &State) -> Option<State> {
        match (current_state, to_state) {
            (State::Idle, State::Calibrate) => Some(State::Calibrate),
            (State::Calibrate, State::Precharge) => Some(State::Precharge),
            (State::Precharge, State::ReadyForLevitation) => Some(State::ReadyForLevitation),
            (State::ReadyForLevitation, State::BeginLevitation) => Some(State::BeginLevitation),
            (State::BeginLevitation, State::Levitating) => Some(State::Levitating),
            (State::Levitating, State::Ready) => Some(State::Ready),
            (State::Ready, State::Accelerate) => Some(State::Accelerate),
            (State::Accelerate, State::LimBrake) => Some(State::LimBrake),
            (State::Accelerate, State::EmergencyBrake) => Some(State::EmergencyBrake),
            (State::LimBrake, State::FrictionBrake) => Some(State::FrictionBrake),
            (State::FrictionBrake, State::StopLevitation) => Some(State::StopLevitation),
            (State::StopLevitation, State::Stopped) => Some(State::Stopped),
            (State::Stopped, State::Safe) => Some(State::Safe),
            (State::EmergencyBrake, State::Safe) => Some(State::Safe),
            (State::Safe, State::Shutdown) => Some(State::Shutdown),
            _ => None,
        }
    }

    pub fn get_macro_state(state: &State) -> MacroState {
        match state {
            State::Idle => MacroState::Idle,
            State::Calibrate => MacroState::Idle,
            State::Precharge => MacroState::Active,
            State::ReadyForLevitation => MacroState::Active,
            State::BeginLevitation => MacroState::Active,
            State::Levitating => MacroState::Active,
            State::Ready => MacroState::Active,
            State::Accelerate => MacroState::Demo,
            State::LimBrake => MacroState::Demo,
            State::FrictionBrake => MacroState::Demo,
            State::StopLevitation => MacroState::Demo,
            State::Stopped => MacroState::Active,
            State::EmergencyBrake => MacroState::Emergency,
            State::Safe => MacroState::Idle,
            State::Shutdown => MacroState::Idle,
        }
    }
}

pub enum MacroState {
    Idle,
    Active,
    Demo,
    Emergency,
}

impl MacroState {
    pub fn to_string(&self) -> String<20> {
        match self {
            MacroState::Idle => String::<20>::from_str("idle").unwrap(),
            MacroState::Active => String::<20>::from_str("active").unwrap(),
            MacroState::Demo => String::<20>::from_str("demo").unwrap(),
            MacroState::Emergency => String::<20>::from_str("emergency").unwrap(),
        }
    }

    pub fn from_string(state: &str) -> Option<MacroState> {
        match state {
            "idle" => Some(MacroState::Idle),
            "active" => Some(MacroState::Active),
            "demo" => Some(MacroState::Demo),
            "emergency" => Some(MacroState::Emergency),
            _ => None,
        }
    }
}
