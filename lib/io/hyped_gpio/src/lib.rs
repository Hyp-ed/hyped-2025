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

    /// A mock GPIO input pin that can be used for testing
    pub struct MockGpioInput {
        current_value: bool,
        next_values: Vec<bool, 10>,
    }

    impl crate::HypedGpioInputPin for MockGpioInput {
        fn is_high(&mut self) -> bool {
            let next_value = self.next_values.pop().unwrap_or(self.current_value);
            self.current_value = next_value;
            self.current_value
        }
    }

    impl MockGpioInput {
        pub fn new(values: Vec<bool, 10>) -> MockGpioInput {
            let mut next_values = values.clone();
            next_values.reverse();
            MockGpioInput {
                current_value: false,
                next_values,
            }
        }
    }

    /// A mock GPIO output pin that can be used for testing
    pub struct MockGpioOutputPin {
        pub current_value: bool,
    }

    impl crate::HypedGpioOutputPin for MockGpioOutputPin {
        fn set_high(&mut self) {
            self.current_value = true;
        }

        fn set_low(&mut self) {
            self.current_value = false;
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
                current_value: false,
            }
        }

        pub fn new_with_value(value: bool) -> MockGpioOutputPin {
            MockGpioOutputPin {
                current_value: value,
            }
        }

        pub fn get_value(&self) -> bool {
            self.current_value
        }
    }
}
