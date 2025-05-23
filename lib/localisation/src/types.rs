use heapless::Vec;

#[derive(PartialEq)]
pub enum SensorChecks {
    Unacceptable,
    Acceptable,
}

pub type RawAccelerometerData<const NUM_ACC: usize, const NUM_AXIS: usize> =
    Vec<Vec<f32, NUM_AXIS>, NUM_ACC>;
pub type AccelerometerData<const NUM_ACC: usize> = Vec<f32, NUM_ACC>;
