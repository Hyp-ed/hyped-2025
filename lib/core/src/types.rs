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

pub const K_NUM_ACCELEROMETERS: i32 = 3;
pub const K_NUM_AXIS: i32 = 3;

pub type RawAccelerometerData = [[f32; K_NUM_AXIS as usize]; K_NUM_ACCELEROMETERS as usize];
pub type AccelerometerData = [f32; K_NUM_ACCELEROMETERS as usize];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bool() {
        assert_eq!(DigitalSignal::from_bool(true), DigitalSignal::High);
        assert_eq!(DigitalSignal::from_bool(false), DigitalSignal::Low);
    }
}
