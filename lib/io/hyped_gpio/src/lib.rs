#![no_std]

/// Abstraction for a GPIO pin so that sensors can be tested with a mock GPIO pin
pub trait HypedGpioInputPin {
    fn is_high(&mut self) -> bool;
}

// Abstraction for a GPIO pin so that actuators can be tested with a mock GPIO pin
pub trait HypedGpioOutputPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

pub mod mock_gpio {
    use heapless::Vec;
    use hyped_core::types::DigitalSignal;

    /// A mock GPIO input pin that can be used for testing
    pub struct MockGpioInput {
        current_value: DigitalSignal,
        next_values: Vec<DigitalSignal, 10>,
    }

    impl crate::HypedGpioInputPin for MockGpioInput {
        fn is_high(&mut self) -> bool {
            let next_value = self.next_values.pop().unwrap_or(self.current_value);
            self.current_value = next_value;
            self.current_value.into()
        }
    }

    impl MockGpioInput {
        pub fn new(values: Vec<DigitalSignal, 10>) -> MockGpioInput {
            let mut next_values = values.clone();
            next_values.reverse();
            MockGpioInput {
                current_value: DigitalSignal::Low,
                next_values,
            }
        }
    }

    /// A mock GPIO output pin that can be used for testing
    pub struct MockGpioOutputPin {
        pub current_value: DigitalSignal,
    }

    impl crate::HypedGpioOutputPin for MockGpioOutputPin {
        fn set_high(&mut self) {
            self.current_value = DigitalSignal::High;
        }

        fn set_low(&mut self) {
            self.current_value = DigitalSignal::Low;
        }
    }

    impl Default for MockGpioOutputPin {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockGpioOutputPin {
        pub fn new() -> MockGpioOutputPin {
            MockGpioOutputPin {
                current_value: DigitalSignal::Low,
            }
        }

        pub fn new_with_value(value: DigitalSignal) -> MockGpioOutputPin {
            MockGpioOutputPin {
                current_value: value,
            }
        }

        pub fn get_value(&self) -> DigitalSignal {
            self.current_value
        }
    }
}
