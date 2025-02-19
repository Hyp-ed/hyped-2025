pub enum FrequencyError {
    Negative(f32),
    Overflow(f32),
}

pub trait FrequencyCalculator {
    fn calculate_frequency(&self, velocity: f32) -> Result<u32, FrequencyError>;
}
