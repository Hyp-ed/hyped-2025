use hyped_core::types::DigitalSignal;
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
    pub fn new(
        sp1_gpio: T,
        sp2_gpio: T,
    ) -> HighPressure<T> {
        HighPressure {
            sp1_gpio,
            sp2_gpio,
        }
    }

    /// Read SP1 and SP2 GPIO pin values and bitwise OR them. Return state of high pressure sensor based on value of OR'd value.
    pub fn get_high_pressure_state(&mut self) -> Result<State, &'static str> {
        let sp1 = self.sp1_gpio.is_high() as u8;
        let sp2 = (self.sp2_gpio.is_high() as u8) << 1;

        let pres_state = sp1 | sp2;

        match pres_state {
            0 => Ok(State::Off),                        // 00 - none are high
            1 => Ok(State::State2),                     // 01 - sp1 is high
            3 => Ok(State::State3),                     // 11 - sp1 and sp2 are high
            _ => Err("ERROR?? Value: {pres_state}."),   // any other case - should never be possible
        }
    }
}

/// Represents the possible state of the high pressure sensor
#[derive(PartialEq, Debug)]
pub enum State {
    Off,
    State2,
    State3
}

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::Vec;
    use hyped_gpio::mock_gpio::MockGpioInput;

    #[test]
    fn test_high_pres_new() {
        let sp1 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::Low]).unwrap());
        let sp2 = MockGpioInput::new(Vec::from_slice(&[DigitalSignal::Low]).unwrap());

        let mut high_pres = HighPressure::new(sp1, sp2);

        assert_eq!(high_pres.get_high_pressure_state(), Ok(State::Off));
    }
}