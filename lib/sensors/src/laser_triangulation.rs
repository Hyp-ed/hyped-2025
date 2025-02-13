use crate::SensorValueRange;
use hyped_adc::HypedAdc;

/// laser_triangulation implements the logic to read current from the RF602 Series Laser Triangulation
/// sensor using the Hyped ADC trait
///
/// Data sheet: https://riftek.com/upload/iblock/61a/zx30dssln4kfve1yaxgwdqnd66bcacw3/Laser_Triangulation_Sensors_RF602_Series_eng.pdf
pub struct LaserTriangulation<T: HypedAdc> {
    adc: T,
    calculate_bounds: fn(f32) -> SensorValueRange<f32>,
}

impl<T: HypedAdc> LaserTriangulation<T> {
    /// Create a new instance of the Laser Triangulation sensor
    pub fn new(adc: T) -> LaserTriangulation<T> {
        Self::new_with_bounds(adc, default_calculate_bounds)
    }

    pub fn new_with_bounds(
        adc: T,
        calculate_bounds: fn(f32) -> SensorValueRange<f32>,
    ) -> LaserTriangulation<T> {
        LaserTriangulation {
            adc,
            calculate_bounds,
        }
    }

    /// The Laser Triangulation sensor has multiple configurations; we're using one where
    /// the base distance is 20mm, measuring range is 25mm. The output amperage is between 4mA and 20mA,
    /// and assuming linearity, the function reads in the amperage and maps it to an appropriate range.
    /// If we change configuration (e.g. different base distance & measuring distance), you'll have to change
    /// the MEASURE_RANGE and BASE_DISTANCE values accordingly. You can find different configuration settings
    /// in the data sheet.
    pub fn read(&mut self) -> SensorValueRange<f32> {
        let current = self.adc.read_value() as f32;
        let result = ((current - AMP_MIN) / (AMP_MAX - AMP_MIN)) * (MEASURE_RANGE) + BASE_DISTANCE;
        (self.calculate_bounds)(result)
    }
}

pub fn default_calculate_bounds(value: f32) -> SensorValueRange<f32> {
    if value <= BASE_DISTANCE || value >= (BASE_DISTANCE + MEASURE_RANGE) {
        SensorValueRange::Critical(value)
    } else {
        SensorValueRange::Safe(value)
    }
}

const BASE_DISTANCE: f32 = 20.0;
const MEASURE_RANGE: f32 = 25.0;
const AMP_MIN: f32 = 0.004;
const AMP_MAX: f32 = 0.02;
