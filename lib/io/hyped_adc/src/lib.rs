#![no_std]

/// ADC trait used to abstract the ADC peripheral
pub trait HypedAdc {
    /// Read value from the ADC channel
    fn read_value(&mut self) -> u16;
    /// Get resolution of ADC
    fn get_resolution(&self) -> u16;
    /// Get reference voltage of ADC pin
    fn get_reference_voltage(&self) -> f32;
    /// Get voltage of the ADC pin
    fn get_voltage(&mut self) -> f32;
}

pub mod mock_adc {
    use core::clone::Clone;
    use heapless::Vec;

    /// A mock ADC instance which can be used for testing
    pub struct MockAdc {
        current_value: u16,
        next_values: Vec<u16, 10>,
        v_ref: f32,
    }

    impl crate::HypedAdc for MockAdc {
        fn read_value(&mut self) -> u16 {
            let next_value: u16 = self.next_values.pop().unwrap_or(self.current_value);
            self.current_value = next_value;
            self.current_value
        }

        fn get_resolution(&self) -> u16 {
            4096
        }

        fn get_reference_voltage(&self) -> f32 {
            self.v_ref
        }

        fn get_voltage(&mut self) -> f32 {
            let adc_reading = self.read_value() as f32;
            let resolution = self.get_resolution() as f32;
            let v_ref = self.get_reference_voltage();
            (adc_reading / resolution) * v_ref
        }
    }

    impl MockAdc {
        pub fn new(mut values: Vec<u16, 10>, v_ref: f32) -> MockAdc {
            let current_value = values.pop().unwrap();
            let mut next_values = values.clone();
            next_values.reverse();
            MockAdc {
                current_value,
                next_values,
                v_ref,
            }
        }
    }
}
