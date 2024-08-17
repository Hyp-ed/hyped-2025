use embassy_stm32::gpio::{Input, Pin};

/**
 * This trait is used to abstract the GPIO pin so that sensors can be tested with a mock GPIO pin.
 */
pub trait GpioPin {
    fn is_high(&mut self) -> bool;
}

/**
 * This struct is used to represent a physical GPIO pin on the STM32 microcontroller.
 */
pub struct EmbassyGpio<P: Pin> {
    pin: Input<'static, P>,
}

impl<P: Pin> GpioPin for EmbassyGpio<P> {
    fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }
}

impl<P: Pin> EmbassyGpio<P> {
    pub fn new(pin: Input<'static, P>) -> Self {
        Self { pin }
    }
}
