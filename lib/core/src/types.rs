use heapless::Vec;

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

pub struct RawAccelerationData {
    x: i32,
    y: i32,
    z: i32,
    timestamp: i32,
    is_sensor_active: bool,
}

pub const K_NUM_ACCELEROMETERS: usize = 4;
pub const K_NUM_AXIS: usize = 3;
pub const K_NUM_ALLOWED_ACCELEROMETER_OUTLIERS: i32 = 2;

#[derive(PartialEq)]
pub enum SensorChecks {
    Unacceptable,
    Acceptable,
}

pub type RawAccelerometerData<const NUM_ACC: usize, const NUM_AXIS: usize> =
    Vec<Vec<f32, NUM_AXIS>, NUM_ACC>;
pub type AccelerometerData<const NUM_ACC: usize> = Vec<f32, NUM_ACC>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bool() {
        assert_eq!(DigitalSignal::from_bool(true), DigitalSignal::High);
        assert_eq!(DigitalSignal::from_bool(false), DigitalSignal::Low);
    }
}
