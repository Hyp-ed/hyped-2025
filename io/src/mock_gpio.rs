use crate::gpio::GpioPin;
use heapless::Vec;

/**
 * This struct is used to represent a mock GPIO pin that can be used for testing.
 */
pub struct MockGpio {
    current_value: bool,
    next_values: Vec<bool, 10>,
}

impl GpioPin for MockGpio {
    fn is_high(&mut self) -> bool {
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
