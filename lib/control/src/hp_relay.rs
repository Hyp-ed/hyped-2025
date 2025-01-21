// need to take in a GPIO pin like Keyence does
// functions: switch_on, switch_off

use hyped_gpio::HypedGpioOutputPin;

pub struct HighPowerRelay<T: HypedGpioOutputPin> {
    gpio: T,
}

impl<T: HypedGpioOutputPin> HighPowerRelay<T> {
    pub fn new(gpio: T) -> HighPowerRelay<T> {
        HighPowerRelay { gpio }
    }

    pub fn switch_on(&mut self) {
        self.gpio.set_high();
    }

    pub fn switch_off(&mut self) {
        self.gpio.set_low();
    }
}
