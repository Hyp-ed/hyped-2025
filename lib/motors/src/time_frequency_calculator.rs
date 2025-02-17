use crate::frequency_calculator::{FrequencyCalculator, FrequencyError};
use embassy_time::Instant;
use heapless::Vec;

/// Takes in a frequency table and calculates the frequency based on the time elapsed since the calculator was created
/// The frequency table is a list of pairs, where the first element is the time elapsed in microseconds,
/// and the second element is the frequency to return when the time elapsed is less than the first element.
///
/// Useful for creating a frequency that predictably changes over time.
///
/// Returns the frequency corresponding to the time elapsed since the calculator was created.
pub struct TimeFrequencyCalculator {
    frequency_table: Vec<(u32, u32), 256>,
    start_time: u32,
}

impl TimeFrequencyCalculator {
    pub fn new(frequency_table: Vec<(u32, u32), 256>) -> Self {
        let start_time = Instant::now().as_micros();
        TimeFrequencyCalculator {
            start_time: start_time as u32,
            frequency_table,
        }
    }
    pub fn reset(&mut self) {
        self.start_time = Instant::now().as_micros() as u32;
    }
}

impl FrequencyCalculator for TimeFrequencyCalculator {
    fn calculate_frequency(&self, _velocity: f32) -> Result<u32, FrequencyError> {
        let microseconds_elapsed = Instant::now().as_micros() as u32 - self.start_time;
        let freq = self
            .frequency_table
            .iter()
            .rev()
            .find(|(time, _)| *time < microseconds_elapsed)
            .map(|(_, frequency)| *frequency)
            .unwrap_or(0);
        Ok(freq)
    }
}
