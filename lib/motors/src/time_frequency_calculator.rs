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
    frequency_table: Vec<Vec<u32, 2>, 256>,
    start_time: u32,
}

impl TimeFrequencyCalculator {
    pub fn new(frequency_table: Vec<Vec<u32, 2>, 256>) -> Self {
        let start_time = Instant::now().as_micros();
        TimeFrequencyCalculator {
            start_time: start_time as u32,
            frequency_table,
        }
    }

    pub fn calculate_frequency(&self) -> u32 {
        let microseconds_elapsed = Instant::now().as_micros() as u32 - self.start_time;
        self.frequency_table
            .iter()
            .rev()
            .find(|pair| pair[0] < microseconds_elapsed)
            .map(|pair| pair[1])
            .unwrap_or(0)
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now().as_micros() as u32;
    }
}
