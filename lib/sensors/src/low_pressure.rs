use hyped_adc::HypedAdc;
use hyped_core::config::SENSORS_CONFIG;

use crate::SensorValueRange;

/// The low pressure sensor (LPS) (model: SPAN-P10R-G18F-PNLK-PNVBA-L1) is able to detect
/// pressure in range from 0 to 10 bar.
///
/// Links to datasheets:
/// - https://www.festo.com/gb/en/a/download-document/datasheet/8134897
/// - https://www.festo.com/media/catalog/203714_documentation.pdf
pub struct LowPressure<T: HypedAdc> {
    adc: T,
    calculate_bounds: fn(f32) -> SensorValueRange<f32>,
}

impl<T: HypedAdc> LowPressure<T> {
    /// Create new low pressure sensor instance
    pub fn new(adc: T) -> LowPressure<T> {
        Self::new_with_bounds(adc, default_calculate_bounds)
    }

    /// Create new low pressure sensor instance with specified bounds
    pub fn new_with_bounds(
        adc: T,
        calculate_bounds: fn(f32) -> SensorValueRange<f32>,
    ) -> LowPressure<T> {
        LowPressure {
            adc,
            calculate_bounds,
        }
    }

    /// Read the pressure (in bar) from the sensor using the ADC.
    /// The conversion rate is expressed as a linear function of:
    ///     pressure = (conversion gradient) * (ADC reading) + (minimum pressure value)
    ///     (y = mx + c0)
    /// where conversion gradient is
    ///     (maximum pressure value - minimum pressure value) / (maximum ADC reading value).
    pub fn read_pressure(&mut self) -> Option<SensorValueRange<f32>> {
        let adc_reading = self.adc.read_value() as f32;
        let adc_resolution = self.adc.get_resolution() as f32;

        // Calculate the pressure in bar
        let pressure_bar: f32 = adc_reading * (MAX_PRESSURE / adc_resolution) + PRESSURE_OFFSET;

        Some((self.calculate_bounds)(pressure_bar))
    }
}

/// Default calculation of the bounds for the low pressure sensor. The bounds are set to:
/// - Safe: 4.0 to 6.0 bar
/// - Warning: 2.0 to 4.0 and 6.0 to 8.0 bar
/// - Critical: below 2.0 and above 8.0 bar
pub fn default_calculate_bounds(value: f32) -> SensorValueRange<f32> {
    if value <= 2.0 || value >= 8.0 {
        SensorValueRange::Critical(value)
    } else if value <= 4.0 || value >= 6.0 {
        SensorValueRange::Warning(value)
    } else {
        SensorValueRange::Safe(value)
    }
}

/// The offset for the pressure value (in bar) read from the sensor.
const PRESSURE_OFFSET: f32 = SENSORS_CONFIG.sensors.low_pressure.pressure_offset as f32;
const MAX_PRESSURE: f32 = SENSORS_CONFIG.sensors.low_pressure.max_pressure as f32;
