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

type AccelerometerData = Vec<RawAccelerationData>;
type RawAccelerometerData = Vec<RawAccelerationData>;

struct RawAccelerationData {
    x: i32,
    y: i32,
    z: i32,
    timestamp: i32,
    is_sensor_active: bool,
}

static kNumAccelerators: i32 = 3;
static kNumAxis: i32 = 3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bool() {
        assert_eq!(DigitalSignal::from_bool(true), DigitalSignal::High);
        assert_eq!(DigitalSignal::from_bool(false), DigitalSignal::Low);
    }
}
