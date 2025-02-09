use embassy_time::Instant;
use heapless::Vec;

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
