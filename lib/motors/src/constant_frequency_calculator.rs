use crate::frequency_calculator::{FrequencyCalculator, FrequencyError};

/// Calculator which takes in a frequency, and always returns the frequency it was initialised with.
pub struct ConstantFrequencyCalculator {
    frequency: u32,
}

impl ConstantFrequencyCalculator {
    pub fn new(frequency: u32) -> Self {
        ConstantFrequencyCalculator { frequency }
    }
}

impl FrequencyCalculator for ConstantFrequencyCalculator {
    fn calculate_frequency(&self, _velocity: f32) -> Result<u32, FrequencyError> {
        Ok(self.frequency)
    }
}
