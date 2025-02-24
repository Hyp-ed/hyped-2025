use hyped_adc::HypedAdc;

use crate::SensorValueRange;

/// The low pressure sensor (LPS) (model: SPAN-P10R-G18F-PNLK-PNVBA-L1) is able to detect
/// pressure in range from 0 to 10 bar. The sensor utilises the ADC protocol to get the
/// pressure value.
///
/// Links to datasheets
///     (https://www.festo.com/gb/en/a/download-document/datasheet/8134897)
///     (https://www.festo.com/media/catalog/203714_documentation.pdf)
pub struct LowPressure<T: HypedAdc> {
    adc: T,
    calculate_bounds: fn(f32) -> SensorValueRange<f32>,
}

const MAX_PRESSURE: f32 = 10.0;
const ADC_RESOLUTION: f32 = 4096.0;
const GRADIENT_LOW: f32 = MAX_PRESSURE / ADC_RESOLUTION;

impl<T: HypedAdc> LowPressure<T> {
    /// Create new low pressure sensor instance
    pub fn new(adc: T) -> LowPressure<T>{
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

    /// Convert ADC reading to bar unit and return value to caller
    /// The conversion rate is expressed as a linear function of:
    ///     pressure = (conversion gradient) * (ADC reading) + (minimum pressure value)
    ///     (y = mx + c0)
    /// where conversion gradient is
    ///     (maximum pressure value - minimum pressure value) / (4096).
    /// 4096 is the maximum ADC reading value.
    /// Since LPS has a minimum pressure of 0 bar, c0 is 0 and was did not need to be included in
    /// the source code.
    /// wrapped in a SensorValueRange enum to indicate if the temperature is safe, warning, or critical.
    pub fn read_pressure(&mut self) -> SensorValueRange<f32> {
        let adc_val = self.adc.read_value() as f32;

        // convert to bar unit
        let bar_pressure_val: f32 = adc_val * GRADIENT_LOW;

        (self.calculate_bounds)(
            bar_pressure_val
        )
    }
}

/// Default calculation of the bounds for the low pressure sensor. The bounds are set to:
/// - Safe: 4.0 to 6.0 bar
/// - Warning: 2.0 to 4.0 and 6.0 to 8.0 bar
/// - Critical: below 2.0 and above 8.0 bar
/// TODO: CHANGE WHEN NEW INFO GOTTEN, THESE ARE PLACEHOLDER VALS
pub fn default_calculate_bounds(value: f32) -> SensorValueRange<f32> {
    if value <= 2.0 || value >= 8.0 {
        SensorValueRange::Critical(value)
    }
    else if value <= 4.0 || value >= 6.0 {
        SensorValueRange::Warning(value)
    }
    else {
        SensorValueRange::Safe(value)
    }
}