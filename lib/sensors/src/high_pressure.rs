use hyped_gpio::HypedGpioInputPin;

/// The high pressure sensor (SPAW-P25R-G12M-2N-M12) is able to detect pressure in range
/// from 0 to 25 bar.
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
    pub fn get_high_pressure_state(&mut self) -> State {
        let sp1 = self.sp1_gpio.is_high() as u8;
        let sp2 = (self.sp2_gpio.is_high() as u8) << 1;

        let pres_state = sp1 | sp2;

        match pres_state {
            0 => State::Off,    // 00 - none are high
            1 => State::State2, // 01 - sp1 is high
            3 => State::State3, // 11 - sp1 and sp2 are high
            _ => State::Off,    // any other case - should never be possible
        }
    }
}

/// Represents the possible state of the high pressure sensor
pub enum State {
    Off,
    State2,
    State3
}
