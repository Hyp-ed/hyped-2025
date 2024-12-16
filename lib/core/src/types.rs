#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub enum SensorValueBounds<T: PartialEq> {
    /// This is the normal range of values for the sensor.
    Safe(T),
    /// Sensor values are outwith the normal range, but not yet critical.
    Warning(T),
    /// This is the range of values that are considered critical and will trigger an emergency.
    Critical(T),
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
