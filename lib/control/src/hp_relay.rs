// need to take in a GPIO pin like Keyence does
// functions: switch_on, switch_off

use hyped_io::gpio::HypedGpioPin;

pub struct HighPowerRelay<T: HypedGpioPin> {
    gpio: T,
};

impl<T> HighPowerRelay<T: HypedGpioPin> {
    pub fn new() {
        todo!()
    }

    pub fn switch_on(&mut self) {
        todo!()
    }

    pub fn switch_off(&mut self) {
        todo!()
    }
}
