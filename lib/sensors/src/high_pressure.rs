use hyped_core::types::DigitalSignal;
use hyped_gpio::HypedGpioInputPin;

use crate::SensorValueRange;

/// The high pressure sensor (SPAW-P25R-G12M-2N-M12) is able to detect pressure in range
/// from 0 to 25 bar.
/// 
/// Links to datasheets
///     (https://www.festo.com/media/catalog/203715_documentation.pdf)
///     (https://ftp.festo.com/public/PNEUMATIC/SOFTWARE_SERVICE/DataSheet/EN_GB/8022773.pdf)

pub struct HighPressure<T: HypedGpioInputPin> {
    gpio: T,
    calculate_bounds: fn(f32) -> SensorValueRange<f32>,
}
impl<T: HypedGpioInputPin> HighPressure<T> {
    /// Create new high pressure sensor instance
    pub fn new(gpio: T) -> HighPressure<T> {
        Self::new_with_bounds(gpio, default_calculate_bounds)
    }

    /// Create new low pressure sensor instance with specified bounds 
    pub fn new_with_bounds(
        gpio: T,
        calculate_bounds: fn(f32) -> SensorValueRange<f32>,
    ) -> HighPressure<T> {
        HighPressure {
            gpio,
            calculate_bounds,
        }
    }

    /// Read pressure (in bar) from high pressure sensor using the ADC.
    /// The conversion rate is expressed a a linear function of:
    ///     pressure = (conversion gradient) * (ADC reading) + (minimum pressure value)
    ///     (y = mx + c0)
    /// where conversion gradient is
    ///     (maximum pressure value - minimum pressure value) / (maximum adc reading value).
    pub fn read_pressure(&mut self) -> Option<SensorValueRange<f32>> {
        //
    }
}

/// Default calculation of the bounds for the high pressure sensor. The bounds are set to:
/// - Safe: 10.0 to 15.0 bar
/// - Warning: 5.0 to 10.0 and 15.0 to 20.0 bar
/// - Critical: below 5.0 and above 20.0 bar
pub fn default_calculate_bounds(value: f32) -> SensorValueRange<f32> {
    if value <= 5.0 || value >= 20.0 {
        SensorValueRange::Critical(value)
    } else if value <= 10.0 || value >= 15.0 {
        SensorValueRange::Warning(value)
    } else {
        SensorValueRange::Safe(value)
    }
}

// offset for pressure value (bar) read from sensor
const PRESSURE_OFFSET: f32 = 0.0;

const MAX_PRESSURE: f32 = 25.0;