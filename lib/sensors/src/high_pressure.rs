use defmt::Format;
use hyped_gpio::HypedGpioInputPin;

/// The high pressure sensor (SPAW-P25R-G12M-2N-M12) is able to detect pressure in range
/// from 0 to 25 bar. The high pressure sensor has 2 switching outputs, SP1 and SP2. SP2
/// can only be high when SP1 is high, meaning that the high pressure sensor has 3 states:
///     Off: Both SP1 and SP2 are LOW
///     2. SP1 is HIGH and SP2 is LOW
///     3. Both SP1 and SP2 are HIGH
///
/// Links to datasheets
///     (https://www.festo.com/media/catalog/203715_documentation.pdf)
///     (https://ftp.festo.com/public/PNEUMATIC/SOFTWARE_SERVICE/DataSheet/EN_GB/8022773.pdf)
pub struct HighPressure<T: HypedGpioInputPin> {
    sp1_gpio: T,
    sp2_gpio: T,
}
impl<T: HypedGpioInputPin> HighPressure<T> {
    /// Create new low pressure sensor instance
    pub fn new(sp1_gpio: T, sp2_gpio: T) -> HighPressure<T> {
        HighPressure { sp1_gpio, sp2_gpio }
    }

    /// Read SP1 and SP2 GPIO pin values and bitwise OR them. Return state of high pressure sensor based on value of OR'd value.
    pub fn get_high_pressure_state(&mut self) -> Result<State, HighPressureError> {
        let sp1 = self.sp1_gpio.is_high();
        let sp2 = self.sp2_gpio.is_high();

        match (sp1, sp2) {
            (false, false) => Ok(State::LowRange),
            (true, false) => Ok(State::MidRange),
            (true, true) => Ok(State::HighRange),
            _ => Err(HighPressureError::InvalidState),  // any other case - should never be possible
        }
    }
}

/// Represents the possible state of the high pressure sensor
#[derive(PartialEq, Debug, Clone, Format)]
pub enum State {
    LowRange,
    MidRange,
    HighRange,
}

/// Represents possible errors for high pressure sensor
#[derive(Debug, PartialEq, Clone)]
pub enum HighPressureError {
    InvalidState,
}

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::Vec;
    use hyped_core::types::DigitalSignal;
    use hyped_gpio::mock_gpio::MockGpioInput;

    #[test]
    fn test_get_high_pressure_state_low_range() {
        let sp1 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::Low]).unwrap());
        let sp2 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::Low]).unwrap());

        let mut high_pres = HighPressure::new(sp1, sp2);

        assert_eq!(high_pres.get_high_pressure_state(), Ok(State::LowRange));
    }

    #[test]
    fn test_get_high_pressure_state_mid_range() {
        let sp1 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::High]).unwrap());
        let sp2 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::Low]).unwrap());

        let mut high_pres = HighPressure::new(sp1, sp2);

        assert_eq!(high_pres.get_high_pressure_state(), Ok(State::MidRange));
    }

    #[test]
    fn test_get_high_pressure_state_high_range() {
        let sp1 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::High]).unwrap());
        let sp2 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::High]).unwrap());

        let mut high_pres = HighPressure::new(sp1, sp2);

        assert_eq!(high_pres.get_high_pressure_state(), Ok(State::HighRange));
    }

    #[test]
    fn test_get_high_pressure_state_error() {
        let sp1 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::Low]).unwrap());
        let sp2 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::High]).unwrap());

        let mut high_pres = HighPressure::new(sp1, sp2);

        assert_eq!(high_pres.get_high_pressure_state(), Err(HighPressureError::InvalidState));
    }
}
