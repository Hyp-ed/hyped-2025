use hyped_gpio::HypedGpioOutputPin;

/// Controls the high power relay by switching it on and off using a GPIO pin.
pub struct HighPowerRelay<'a, T: HypedGpioOutputPin> {
    gpio: &'a mut T,
}

impl<'a, T: HypedGpioOutputPin> HighPowerRelay<'a, T> {
    pub fn new(gpio: &'a mut T) -> HighPowerRelay<'a, T> {
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

#[cfg(test)]
mod tests {
    use super::HighPowerRelay;
    use hyped_core::types::DigitalSignal;
    use hyped_gpio::mock_gpio::MockGpioOutputPin;

    #[test]
    fn test_switch_on() {
        let mut mock_gpio_output_pin = MockGpioOutputPin::default();
        let mut relay = HighPowerRelay::new(&mut mock_gpio_output_pin);
        relay.switch_on();
        assert_eq!(mock_gpio_output_pin.get_value(), DigitalSignal::High);
    }

    #[test]
    fn test_switch_off() {
        let mut mock_gpio_output_pin = MockGpioOutputPin::default();
        let mut relay = HighPowerRelay::new(&mut mock_gpio_output_pin);
        relay.switch_off();
        assert_eq!(mock_gpio_output_pin.get_value(), DigitalSignal::Low);
    }
}
