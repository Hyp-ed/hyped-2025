use heapless::Vec;

pub const K_NUM_ACCELEROMETERS: usize = 4;
pub const K_NUM_AXIS: usize = 3;
pub const K_NUM_ALLOWED_ACCELEROMETER_OUTLIERS: usize = 2;

#[derive(PartialEq)]
pub enum SensorChecks {
    Unacceptable,
    Acceptable,
}

pub type RawAccelerometerData<const NUM_ACC: usize, const NUM_AXIS: usize> =
    Vec<Vec<f32, NUM_AXIS>, NUM_ACC>;
pub type AccelerometerData<const NUM_ACC: usize> = Vec<f32, NUM_ACC>;
