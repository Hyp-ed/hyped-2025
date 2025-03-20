use hyped_core::types::DigitalSignal;
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
    calculate_bounds: fn(f32) -> SensorValueRange<f32>,
}
impl<T: HypedGpioInputPin> HighPressure<T> {
    /// Create new high pressure sensor instance
    pub fn new(
        sp1_gpio: T,
        sp2_gpio: T,
    ) -> HighPressure<T> {
        Self::new_with_bounds(sp1_gpio, sp2_gpio, default_calculate_bounds)
    }

    /// Create new low pressure sensor instance with specified bounds 
    pub fn new_with_bounds(
        sp1_gpio: T,
        sp2_gpio: T,
        calculate_bounds: fn(f32) -> SensorValueRange<f32>,
    ) -> HighPressure<T> {
        HighPressure {
            sp1_gpio,
            sp2_gpio,
            calculate_bounds,
        }
    }

    /// Read SP1 and SP2 GPIO pin values and bit OR them. Return state of high pressure sensor based on value of OR'd value.
    pub fn get_high_pressure_state(&mut self) -> State {
        // get pressure from BOTHHH gpio pins
        // return enum variant corresponding to which 3 states its in
        // states are affected by which pins are past their thresholds
        let sp1 = self.sp1_gpio.is_high() as u8;
        let sp2 = (self.sp2_gpio.is_high() as u8) << 1;

        let pres_state = sp1 | sp2;

        match pres_state {
            0 => State::Off,
            1 => State::State2,
            3 => State::State3,
        }
    }
}

// offset for pressure value (bar) read from sensor
const PRESSURE_OFFSET: f32 = 0.0;

const MAX_PRESSURE: f32 = 25.0;

/// Represents the possible state of the high pressure sensor
pub enum State {
    Off,
    State2,
    State3
}
