use hyped_adc::HypedAdc;

/// The low pressure sensor ([MODEL]) is able to detect pressure in range
/// from [RANGE] bar.
/// 

/// Links to datasheets
///     [LINKS]

pub struct LowPressure<T: HypedAdc> {
    pressure: u16,
    adc: T
}

impl<T: HypedAdc> LowPressure<T> {
    /// Create new high pressure sensor instance
    pub fn new(adc: T) -> LowPressure<T> {
        LowPressure {
            pressure: 0,
            adc,
        }
    }

    /// Read pressure from high pressure sensor
    pub fn read_pressure(&mut self) -> u16 {
        let pressure_val = self.adc.read_value();
        self.pressure = pressure_val;

        self.pressure
    }
}

#[cfg(test)]
mod tests {
    
}