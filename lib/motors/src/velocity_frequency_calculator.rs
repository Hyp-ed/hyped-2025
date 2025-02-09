pub struct VelocityFrequencyCalculator {
    coefficients: [f32; 5],
}

pub enum FrequencyError {
    Negative(f32),
    Overflow(f32),
}

impl VelocityFrequencyCalculator {
    pub fn new(coefficients: [f32; 5]) -> Self {
        VelocityFrequencyCalculator { coefficients }
    }

    pub fn calculate_frequency(&self, velocity: f32) -> Result<u32, FrequencyError> {
        let frequency = velocity * velocity * velocity * velocity * self.coefficients[0]
            + velocity * velocity * velocity * self.coefficients[1]
            + velocity * velocity * self.coefficients[2]
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
