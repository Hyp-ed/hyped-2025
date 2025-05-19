use crate::SensorValueRange;
use hyped_adc::HypedAdc;
use hyped_core::config::SENSORS_CONFIG;

/// laser_triangulation implements the logic to read current from the RF602 Series Laser Triangulation
/// sensor using the Hyped ADC trait.
///
/// Data sheet PDF is in the HYPED Slack and Google Drive (no longer available online).
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
    pub fn read(&mut self) -> Result<SensorValueRange<f32>, LaserTriangulationError> {
        let voltage = self.adc.get_voltage();
        let v_ref = self.adc.get_reference_voltage();
        if voltage - ZERO_OFFSET < 0.1 {
            return Err(LaserTriangulationError::OutOfRange);
        }
        let result =
            ((voltage - ZERO_OFFSET) / (v_ref - ZERO_OFFSET)) * MEASURE_RANGE + BASE_DISTANCE;
        Ok((self.calculate_bounds)(result))
    }
}

pub fn default_calculate_bounds(value: f32) -> SensorValueRange<f32> {
    if value <= BASE_DISTANCE + 1.0 || value >= (BASE_DISTANCE + MEASURE_RANGE) - 1.0 {
        SensorValueRange::Warning(value)
    } else {
        SensorValueRange::Safe(value)
    }
}

pub enum LaserTriangulationError {
    OutOfRange,
}

const BASE_DISTANCE: f32 = SENSORS_CONFIG.sensors.laser_triangulation.base_distance as f32;
const MEASURE_RANGE: f32 = SENSORS_CONFIG.sensors.laser_triangulation.measure_range as f32;
const ZERO_OFFSET: f32 = SENSORS_CONFIG.sensors.laser_triangulation.zero_offset as f32;
