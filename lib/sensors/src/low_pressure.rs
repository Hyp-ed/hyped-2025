use hyped_adc::HypedAdc;

/// The low pressure sensor (LPS) (model: SPAN-P10R-G18F-PNLK-PNVBA-L1) is able to detect
/// pressure in range from 0 to 10 bar. The sensor utilises the ADC protocol to get the
/// pressure value, and its conversion rate is expressed as a linear function of:
///     pressure = (conversion gradient) * (ADC reading) + (minimum pressure value)
///     (y = mx + c0)
/// where conversion gradient is
///     (maximum pressure value + minimum pressure value) / (4096).
/// 4096 is the maximum ADC reading value.
/// Since LPS has a minimum pressure of 0 bar, c0 is 0 and was did not need to be included in
/// the source code.
///
/// Links to datasheets
///     (https://www.festo.com/gb/en/a/download-document/datasheet/8134897)
///     (https://www.festo.com/media/catalog/203714_documentation.pdf)
///
/// low pressure sensor struct
pub struct LowPressure<T: HypedAdc> {
    adc: T,
}

const GRADIENT_LOW: f32 = 0.00244;

impl<T: HypedAdc> LowPressure<T> {
    /// Create new low pressure sensor instance
    pub fn new(adc: T) -> LowPressure<T> {
        LowPressure { adc }
    }

    /// Convert ADC reading to bar unit and return value to caller
    pub fn read_pressure(&mut self) -> f32 {
        let adc_val = self.adc.read_value() as f32;

        // convert to bar unit
        let bar_pressure_val: f32 = adc_val * GRADIENT_LOW;

        bar_pressure_val
    }
}
