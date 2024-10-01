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

#[derive(Hash, PartialEq, Eq)]
pub struct SourceAndTarget {
    pub(crate) source: State,
    pub(crate) target: State,
}
