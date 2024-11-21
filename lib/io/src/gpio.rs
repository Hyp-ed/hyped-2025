/// Abstraction for a GPIO pin so that sensors can be tested with a mock GPIO pin
pub trait HypedGpioPin {
    fn is_high(&mut self) -> bool;
    fn switch_on(&mut self) -> bool;
    fn switch_off(&mut self) -> bool;
}

pub mod mock_gpio {
    use heapless::Vec;

    /// A mock GPIO pin that can be used for testing
    pub struct MockGpio {
        current_value: bool,
        next_values: Vec<bool, 10>,
    }

    impl crate::gpio::HypedGpioPin for MockGpio {
        fn is_high(&mut self) -> bool {
            let next_value = self.next_values.pop().unwrap_or(self.current_value);
            self.current_value = next_value;
            self.current_value
        }

        fn switch_on(&mut self) -> bool {
            let next_value = self.next_values.pop().unwrap_or(self.current_value);
            self.current_value = next_value;
            self.current_value
        }

        fn switch_off(&mut self) -> bool {
            let next_value = self.next_values.pop().unwrap_or(self.current_value);
            self.current_value = next_value;
            self.current_value
        }
    }

    impl MockGpio {
        pub fn new(values: Vec<bool, 10>) -> MockGpio {
            let mut next_values = values.clone();
            next_values.reverse();
            MockGpio {
                current_value: false,
                next_values,
            }
        }
    }
}
