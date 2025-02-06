use hyped_adc::HypedAdc;

/// The low pressure sensor ([MODEL]) is able to detect pressure in range
/// from [RANGE] bar.
/// 

/// Links to datasheets
///     (https://www.festo.com/gb/en/a/download-document/datasheet/8134897)
///     (https://www.festo.com/media/catalog/203714_documentation.pdf)

pub struct LowPressure<T: HypedAdc> {
    pressure: u16,
    adc: T
}

impl<T: HypedAdc> LowPressure<T> {
    /// Create new low pressure sensor instance
    pub fn new(adc: T) -> LowPressure<T> {
        LowPressure {
            pressure: 0,
            adc,
        }
    }

    /// Read pressure from low pressure sensor
    pub fn read_pressure(&mut self) -> u16 {
        let pressure_val = self.adc.read_value();
        self.pressure = pressure_val;

        self.pressure
    }
}

#[cfg(test)]
mod tests {
    
}