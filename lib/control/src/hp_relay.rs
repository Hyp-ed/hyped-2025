use hyped_gpio::HypedGpioOutputPin;

/// Controls the high power relay by switching it on and off using a GPIO pin.
pub struct HighPowerRelay<T: HypedGpioOutputPin> {
    gpio: T,
}

impl<T: HypedGpioOutputPin> HighPowerRelay<T> {
    pub fn new(gpio: T) -> HighPowerRelay<T> {
        HighPowerRelay { gpio }
    }

    /// Switches the relay on by setting the GPIO pin high.
    pub fn switch_on(&mut self) {
        self.gpio.set_high();
    }

    /// Switches the relay off by setting the GPIO pin low.
    pub fn switch_off(&mut self) {
        self.gpio.set_low();
    }
}
