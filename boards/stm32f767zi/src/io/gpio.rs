use embassy_stm32::gpio::Input;
use hyped_io::gpio::GpioPin;

/// A GPIO pin on the STM32L476RG.
pub struct Stm32f767ziGpio {
    pin: Input<'static>,
}

impl GpioPin for Stm32f767ziGpio {
    fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }
}

impl Stm32f767ziGpio {
    /// Create a new instance of our GPIO implementation for the STM32L476RG
    pub fn new(pin: Input<'static>) -> Self {
        Self { pin }
    }
}
