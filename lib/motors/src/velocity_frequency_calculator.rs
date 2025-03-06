use crate::frequency_calculator::{FrequencyCalculator, FrequencyError};
use libm::powf;

/// Calculates the frequency by taking in a velocity and using a polynomial to calculate the frequency.
/// The polynomial is defined by the coefficients array
///
/// Returns the frequency calculated from the velocity or an error if the frequency is negative or overflows
pub struct VelocityFrequencyCalculator {
    coefficients: [f32; 5],
}

impl VelocityFrequencyCalculator {
    pub fn new(coefficients: [f32; 5]) -> Self {
        VelocityFrequencyCalculator { coefficients }
    }
}

impl FrequencyCalculator for VelocityFrequencyCalculator {
    fn calculate_frequency(&self, velocity: f32) -> Result<u32, FrequencyError> {
        let frequency = powf(velocity, 4.0) * self.coefficients[0]
            + powf(velocity, 3.0) * self.coefficients[1]
            + powf(velocity, 2.0) * self.coefficients[2]
            + velocity * self.coefficients[3]
            + self.coefficients[4];

        if frequency < 0.0 {
            return Err(FrequencyError::Negative(frequency));
        }

        if frequency > u32::MAX as f32 {
            return Err(FrequencyError::Overflow(frequency));
        }

        Ok(frequency as u32)
    }
}
