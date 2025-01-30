#[derive(PartialEq, Debug, Clone, Copy)]
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

impl From<DigitalSignal> for bool {
    fn from(signal: DigitalSignal) -> bool {
        match signal {
            DigitalSignal::High => true,
            DigitalSignal::Low => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bool() {
        assert_eq!(DigitalSignal::from_bool(true), DigitalSignal::High);
        assert_eq!(DigitalSignal::from_bool(false), DigitalSignal::Low);
    }
}
