#[derive(PartialEq)]
pub enum DigitalSignal {
    High,
    Low,
}

impl DigitalSignal {
    pub fn from_bool(signal: bool) -> DigitalSignal {
        if signal {
            DigitalSignal::High
        } else {
            DigitalSignal::Low
        }
    }
}
