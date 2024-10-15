use embassy_stm32::gpio::Input;
use hyped_io::gpio::GpioPin;

/// A GPIO pin on the STM32L476RG.
pub struct Stm32l476rgGpio {
    pin: Input<'static>,
}

impl GpioPin for Stm32l476rgGpio {
    fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }
}

impl Stm32l476rgGpio {
    /// Create a new instance of our GPIO implementation for the STM32L476RG
    pub fn new(pin: Input<'static>) -> Self {
        Self { pin }
    }
}
