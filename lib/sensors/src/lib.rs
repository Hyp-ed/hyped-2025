#![cfg_attr(not(test), no_std)]

pub mod keyence;
pub mod temperature;
pub mod time_of_flight;

#[must_use]
#[derive(PartialEq, Debug, Clone)]
pub enum SensorValueRange<T: PartialEq> {
    /// This is the normal range of values for the sensor.
    Safe(T),
    /// Sensor values are outwith the normal range, but not yet critical.
    Warning(T),
    /// This is the range of values that are considered critical and will trigger an emergency.
    Critical(T),
}

macro_rules! create_calculate_bounds_function {
    ($critical_limit_low:expr, $warning_limit_low:expr, $warning_limit_high:expr, $critical_limit_high:expr) => {
        pub fn calculate_bounds(value: f32) -> SensorValueRange<f32> {
            if value <= $critical_limit_low || value >= $critical_limit_high {
                SensorValueRange::Critical(value)
            } else if value <= $warning_limit_low || value >= $warning_limit_high {
                SensorValueRange::Warning(value)
            } else {
                SensorValueRange::Safe(value)
            }
        }
    };
}
