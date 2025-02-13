/// Calculator which takes in a frequency, and always returns the frequency it was initialised with.
pub struct ConstantFrequencyCalculator {
    frequency: u32,
}

impl ConstantFrequencyCalculator {
    pub fn new(frequency: u32) -> Self {
        ConstantFrequencyCalculator { frequency }
    }

    pub fn calculate_frequency(&self) -> u32 {
        self.frequency
    }
}
