#![no_std]

/// ADC trait used to abstract the ADC peripheral
pub trait HypedAdc {
    fn read_value(&mut self) -> u16;
    fn get_resolution(&self) -> u16;
}

pub mod mock_adc {
    use core::clone::Clone;
    use heapless::Vec;

    /// A mock ADC instance which can be used for testing
    pub struct MockAdc {
        current_value: u16,
        next_values: Vec<u16, 10>,
    }

    impl crate::HypedAdc for MockAdc {
        /// Reads a value from the ADC
        fn read_value(&mut self) -> u16 {
            let next_value: u16 = self.next_values.pop().unwrap_or(self.current_value);
            self.current_value = next_value;
            self.current_value
        }

        /// Return value of resolution (4095)
        fn get_resolution(&self) -> u16 {
            4095
        }
    }

    impl MockAdc {
        pub fn new(mut values: Vec<u16, 10>) -> MockAdc {
            let current_value = values.pop().unwrap();
            let mut next_values = values.clone();
            next_values.reverse();
            MockAdc {
                current_value,
                next_values,
            }
        }
    }
}
