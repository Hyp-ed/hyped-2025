use hyped_adc::HypedAdc;

/// The low pressure sensor (LPS) (model: SPAN-P10R-G18F-PNLK-PNVBA-L1) is able to detect
/// pressure in range from 0 to 10 bar. The sensor utilises the ADC protocol to get the
/// pressure value, and its conversion rate is expressed as a linear function of:
///  

/// Links to datasheets
///     (https://www.festo.com/gb/en/a/download-document/datasheet/8134897)
///     (https://www.festo.com/media/catalog/203714_documentation.pdf)

pub struct LowPressure<T: HypedAdc> {
    adc: T
}

impl<T: HypedAdc> LowPressure<T> {
    /// Create new low pressure sensor instance
    pub fn new(adc: T) -> LowPressure<T> {
        LowPressure {
            adc
        }
    }

    /// Convert ADC reading to bar unit and assign to pressure variable
    pub fn read_pressure(&mut self) -> u16 {
        // conversion gradient value
        const GRADIENT_LOW: f32 = 0.00244;

        // read ADC
        let adc_val = self.adc.read_value();
        
        // convert to bar unit
        let bar_pressure_val: u16 = adc_val * GRADIENT_LOW;


        bar_pressure_val
    }
}

#[cfg(test)]
mod tests {
    
}