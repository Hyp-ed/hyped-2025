#![no_std]

pub mod keyence;
pub mod temperature;

#[must_use]
#[derive(PartialEq, Debug)]
pub enum SensorValueRange<T: PartialEq> {
    /// This is the normal range of values for the sensor.
    Safe(T),
    /// Sensor values are outwith the normal range, but not yet critical.
    Warning(T),
    /// This is the range of values that are considered critical and will trigger an emergency.
    Critical(T),
}
