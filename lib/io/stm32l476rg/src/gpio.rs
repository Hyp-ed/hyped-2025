use embassy_stm32::gpio::{Input, Pin};
use hyped_io::gpio::GpioPin;

/// A GPIO pin on the STM32L476RG.
pub struct Stm32l476rgGpio<P: Pin> {
    pin: Input<'static, P>,
}

impl<P: Pin> GpioPin for Stm32l476rgGpio<P> {
    fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }
}

impl<P: Pin> Stm32l476rgGpio<P> {
    pub fn new(pin: Input<'static, P>) -> Self {
        Self { pin }
    }
}
