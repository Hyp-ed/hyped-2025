use hyped_gpio::HypedGpioOutputPin;

/// Wrapper around a GPIO pin that is used as a Chip Select (CS) pin for an SPI device
pub struct HypedSpiCsPin<P: HypedGpioOutputPin> {
    pin: P,
}

impl<P: HypedGpioOutputPin> HypedSpiCsPin<P> {
    /// Create a new Chip Select pin
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Set the Chip Select pin to the active state (low)
    pub fn set_active(&mut self) {
        self.pin.set_low();
    }

    /// Set the Chip Select pin to the inactive state (high)
    pub fn set_inactive(&mut self) {
        self.pin.set_high();
    }
}

impl<P: HypedGpioOutputPin> Drop for HypedSpiCsPin<P> {
    /// Set the Chip Select pin to the inactive state when the Chip Select pin is dropped
    fn drop(&mut self) {
        self.set_inactive();
    }
}
