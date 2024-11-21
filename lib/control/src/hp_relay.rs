// need to take in a GPIO pin like Keyence does
// functions: switch_on, switch_off

use hyped_io::gpio::HypedGpioPin;

pub struct HighPowerRelay<T: HypedGpioPin> {
    gpio: T,
}

impl<T: HypedGpioPin> HighPowerRelay<T> {
    pub fn new(gpio: T) -> HighPowerRelay<T> {
        HighPowerRelay { gpio }
    }

    pub fn switch_on(&mut self) {
        self.gpio.switch_on();
    }

    pub fn switch_off(&mut self) {
        self.gpio.switch_off();
    }
}
