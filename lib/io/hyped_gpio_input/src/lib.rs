#![no_std]

/// Abstraction for a GPIO pin so that sensors can be tested with a mock GPIO pin
pub trait HypedGpioInput {
    fn is_high(&mut self) -> bool;
}

pub trait GpioOutputPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
}

pub mod mock_gpio {
    use heapless::Vec;

    /// A mock GPIO pin that can be used for testing
    pub struct MockGpioInput {
        current_value: bool,
        next_values: Vec<bool, 10>,
    }

    impl crate::HypedGpioInput for MockGpioInput {
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
}
