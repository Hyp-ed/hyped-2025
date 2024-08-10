use crate::types::DigitalSignal;
use embassy_stm32::gpio::{Input, Pin};

pub struct Keyence<'p, P: Pin> {
    stripe_count: u32,
    last_signal: DigitalSignal,
    gpio_input: Input<'p, P>,
}

impl<'p, P: Pin> Keyence<'p, P> {
    pub fn new(gpio_input: Input<'p, P>) -> Keyence<'p, P> {
        Keyence {
            stripe_count: 0,
            last_signal: DigitalSignal::Low,
            gpio_input,
        }
    }

    pub fn get_stripe_count(&self) -> u32 {
        self.stripe_count
    }

    pub fn update_stripe_count(&mut self) -> Result<(), ()> {
        let current_signal = DigitalSignal::from_bool(self.gpio_input.is_high());
        if current_signal == DigitalSignal::High && self.last_signal == DigitalSignal::Low {
            self.stripe_count += 1;
        }
        self.last_signal = current_signal;
        return Ok(());
    }
}
