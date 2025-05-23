use hyped_i2c::{HypedI2c, I2cError};

use crate::SensorValueRange;

/// Temperature implements the logic to read the temperature from the STTS22H temperature sensor
/// using the I2C peripheral provided by the HypedI2c trait.
///
/// The temperature sensor is configured to continuous mode with a sampling rate of 200Hz.
/// The temperature is read from the sensor and converted to a floating point value in degrees Celsius.
///
/// Data sheet: https://www.st.com/resource/en/datasheet/stts22h.pdf
pub struct Temperature<'a, T: HypedI2c> {
    i2c: &'a mut T,
    device_address: u8,
    calculate_bounds: fn(f32) -> SensorValueRange<f32>,
}

impl<'a, T: HypedI2c> Temperature<'a, T> {
    /// Create a new instance of the temperature sensor and attempt to configure it
    pub fn new(
        i2c: &'a mut T,
        device_address: TemperatureAddresses,
    ) -> Result<Self, TemperatureError> {
        Self::new_with_bounds(i2c, device_address, default_calculate_bounds)
    }

    /// Create a new instance of the temperature sensor with the specified bounds and attempt to configure it
    pub fn new_with_bounds(
        i2c: &'a mut T,
        device_address: TemperatureAddresses,
        calculate_bounds: fn(f32) -> SensorValueRange<f32>,
    ) -> Result<Self, TemperatureError> {
        // Set up the temperature sensor by sending the configuration settings to the STTS22H_CTRL register
        let device_address = device_address as u8;
        match i2c.write_byte_to_register(device_address, STTS22H_CTRL, STTS22H_CONFIG_SETTINGS) {
            Ok(_) => Ok(Self {
                i2c,
                device_address,
                calculate_bounds,
            }),
            Err(e) => Err(TemperatureError::I2cError(e)),
        }
    }

    /// Read the temperature from the sensor and return it as a floating point value in degrees Celsius.
    /// Returns None if there was an error reading the temperature, otherwise returns the temperature
    /// wrapped in a SensorValueRange enum to indicate if the temperature is safe, warning, or critical.
    pub fn read(&mut self) -> Option<SensorValueRange<f32>> {
        // Read the high and low bytes of the temperature and combine them to get the temperature
        let temperature_high_byte = self
            .i2c
            .read_byte(self.device_address, STTS22H_DATA_TEMP_H)?;
        let temperature_low_byte = self
            .i2c
            .read_byte(self.device_address, STTS22H_DATA_TEMP_L)?;

        let combined: f32 =
            (((temperature_high_byte as u16) << 8) | temperature_low_byte as u16) as f32;

        // Check if the temperature is negative
        if combined >= TWO_POWER_15 {
            // Convert the temperature to a negative value
            return Some((self.calculate_bounds)(
                (combined - TWO_POWER_16) * STTS22H_TEMP_SCALING_FACTOR,
            ));
        }

        Some((self.calculate_bounds)(
            combined * STTS22H_TEMP_SCALING_FACTOR,
        ))
    }

    /// Check the status of the temperature sensor
    pub fn check_status(&mut self) -> Status {
        match self.i2c.read_byte(self.device_address, STTS22H_STATUS) {
            Some(byte) => Status::from_byte(byte),
            None => Status::Unknown,
        }
    }
}

/// Represents the possible I2C addresses for the STTS22H temperature sensor
#[repr(u8)]
#[allow(dead_code)]
pub enum TemperatureAddresses {
    Address3f = 0x3f,
    // Other possible addresses
    Address38 = 0x38,
    Address3c = 0x3c,
    Address3e = 0x3e,
}

/// Represents the possible errors that can occur when reading the temperature sensor
#[derive(Debug)]
pub enum TemperatureError {
    I2cError(I2cError),
}

/// Represents the possible statuses of the temperature sensor
#[derive(Debug, PartialEq)]
pub enum Status {
    Busy,
    TempOverUpperLimit,
    TempUnderLowerLimit,
    Ok,
    Unknown,
}

impl Status {
    /// Convert a byte read from the STTS22H_STATUS register to a Status enum
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            STTS22H_STATUS_BUSY => Self::Busy,
            STTS22H_TEMP_OVER_UPPER_LIMIT => Self::TempOverUpperLimit,
            STTS22H_TEMP_UNDER_LOWER_LIMIT => Self::TempUnderLowerLimit,
            _ => Self::Ok,
        }
    }
}

/// Default calculation of the bounds for the temperature sensor. The bounds are set to:
/// - Safe: 20.0 to 80.0 degrees Celsius
/// - Warning: 0.0 to 20.0 and 80.0 to 100.0 degrees Celsius
/// - Critical: Below 0.0 and above 100.0 degrees Celsius
pub fn default_calculate_bounds(value: f32) -> SensorValueRange<f32> {
    if value <= 0.0 || value >= 100.0 {
        SensorValueRange::Critical(value)
    } else if value <= 20.0 || value >= 80.0 {
        SensorValueRange::Warning(value)
    } else {
        SensorValueRange::Safe(value)
    }
}

// Registers for the STTS22H temperature sensor
const STTS22H_CTRL: u8 = 0x04;
const STTS22H_DATA_TEMP_L: u8 = 0x06;
const STTS22H_DATA_TEMP_H: u8 = 0x07;
const STTS22H_STATUS: u8 = 0x05;

// Values to check the status of the temperature sensor from the STTS22H_STATUS register
const STTS22H_STATUS_BUSY: u8 = 0x01;
const STTS22H_TEMP_OVER_UPPER_LIMIT: u8 = 0x02;
const STTS22H_TEMP_UNDER_LOWER_LIMIT: u8 = 0x04;

// These settings set the sensor to continuous mode, sets IF_ADD_INC, and sets the sampling rate to 200Hz
const STTS22H_CONFIG_SETTINGS: u8 = 0x3c;

// Scaling factor to convert the temperature from the sensor to degrees Celsius
const STTS22H_TEMP_SCALING_FACTOR: f32 = 0.01;
const TWO_POWER_15: f32 = 32768.0;
const TWO_POWER_16: f32 = 65536.0;

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    use super::*;
    use embassy_sync::blocking_mutex::Mutex;
    use heapless::FnvIndexMap;
    use hyped_i2c::mock_i2c::MockI2c;

    #[test]
    fn test_write_config() {
        let i2c_values = Mutex::new(RefCell::new(FnvIndexMap::new()));
        let mut i2c = MockI2c::new(&i2c_values);
        let _ = Temperature::new(&mut i2c, TemperatureAddresses::Address3f).unwrap();
        let i2c_value = i2c
            .get_writes()
            .get(&(TemperatureAddresses::Address3f as u8, STTS22H_CTRL.into()))
            .cloned();
        assert_eq!(i2c_value, Some(Some(STTS22H_CONFIG_SETTINGS)));
    }

    #[test]
    fn test_temperature_read_0() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (
                TemperatureAddresses::Address3f as u8,
                STTS22H_DATA_TEMP_H as u16,
            ),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (
                TemperatureAddresses::Address3f as u8,
                STTS22H_DATA_TEMP_L as u16,
            ),
            Some(0x00),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut temperature = Temperature::new(&mut i2c, TemperatureAddresses::Address3f).unwrap();
        assert_eq!(temperature.read(), Some(SensorValueRange::Critical(0.0)));
    }

    #[test]
    fn test_temperature_read_25() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (
                TemperatureAddresses::Address3f as u8,
                STTS22H_DATA_TEMP_H as u16,
            ),
            Some(0x09),
        );
        let _ = i2c_values.insert(
            (
                TemperatureAddresses::Address3f as u8,
                STTS22H_DATA_TEMP_L as u16,
            ),
            Some(0xc4),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut temperature = Temperature::new(&mut i2c, TemperatureAddresses::Address3f).unwrap();
        assert_eq!(temperature.read(), Some(SensorValueRange::Safe(25.0)));
    }

    #[test]
    fn test_temperature_read_minus_10() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (
                TemperatureAddresses::Address3f as u8,
                STTS22H_DATA_TEMP_H as u16,
            ),
            Some(0xfc),
        );
        let _ = i2c_values.insert(
            (
                TemperatureAddresses::Address3f as u8,
                STTS22H_DATA_TEMP_L as u16,
            ),
            Some(0x18),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut temperature = Temperature::new(&mut i2c, TemperatureAddresses::Address3f).unwrap();
        assert_eq!(temperature.read(), Some(SensorValueRange::Critical(-10.0)));
    }

    #[test]
    fn test_temperature_status_busy() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (TemperatureAddresses::Address3f as u8, STTS22H_STATUS.into()),
            Some(STTS22H_STATUS_BUSY),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut temperature = Temperature::new(&mut i2c, TemperatureAddresses::Address3f).unwrap();
        assert_eq!(temperature.check_status(), Status::Busy);
    }

    #[test]
    fn test_temperature_status_temp_over_upper_limit() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (TemperatureAddresses::Address3f as u8, STTS22H_STATUS.into()),
            Some(STTS22H_TEMP_OVER_UPPER_LIMIT),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c: MockI2c<'_> = MockI2c::new(&i2c_values);
        let mut temperature = Temperature::new(&mut i2c, TemperatureAddresses::Address3f).unwrap();
        assert_eq!(temperature.check_status(), Status::TempOverUpperLimit);
    }

    #[test]
    fn test_temperature_status_temp_under_lower_limit() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (TemperatureAddresses::Address3f as u8, STTS22H_STATUS.into()),
            Some(STTS22H_TEMP_UNDER_LOWER_LIMIT),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c: MockI2c<'_> = MockI2c::new(&i2c_values);
        let mut temperature = Temperature::new(&mut i2c, TemperatureAddresses::Address3f).unwrap();
        assert_eq!(temperature.check_status(), Status::TempUnderLowerLimit);
    }
}
