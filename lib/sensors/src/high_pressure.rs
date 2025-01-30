use hyped_adc::HypedAdc;

/// The high pressure sensor (SPAW-P25R-G12M-2N-M12) is able to detect pressure in range from 0 to 25 bar
/// 

/// Links to datasheets
///     (https://www.festo.com/media/catalog/203715_documentation.pdf)
///     (https://ftp.festo.com/public/PNEUMATIC/SOFTWARE_SERVICE/DataSheet/EN_GB/8022773.pdf)

pub struct HighPressure<T: HypedAdc> {
    pressure: u16,
    adc: T
}

impl<T: HypedAdc> HighPressure<T> {
    /// Creates new high pressure sensor instance
    pub fn new(adc: T) -> HighPressure<T> {
        HighPressure {
            pressure: 0,
            adc,
        }
    }

    /// Read pressure from 
    pub fn read_pressure(&mut self) -> u16 {
        let pressure_val = self.adc.read_value();
        self.pressure = pressure_val;

        self.pressure
    }
}

#[cfg(test)]
mod tests {

}