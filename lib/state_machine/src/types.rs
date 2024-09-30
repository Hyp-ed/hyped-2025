#[derive(Hash)]
pub enum State {
    KIdle,
    KCalibrate,
    KPrecharge,
    KReadyForLevitation,
    KBeginLevitation,
    KLevitating,
    KReady,
    KAccelerate,
    KLimBrake,
    KFrictionBrake,
    KStopLevitation,
    KStopped,
    KBatteryRecharge,
    KCapacitorDischarge,
    KFailureBrake,
    KFailure,
    KSafe,
    KShutdown,
}

#[derive(Hash)]
pub struct SourceAndTarget {
    source: State,
    target: State,
}
