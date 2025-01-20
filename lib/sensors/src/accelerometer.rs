use hyped_i2c::{i2c_write_or_err, HypedI2c, I2cError};

use crate::SensorValueRange;

/// Accelerometer implements the logic to read the temperature from the LIS2DS12 accelerometer
/// using the peripheral provided by the HypedI2c trait.
///
/// Based on last year's implementation (might need to change this later),
/// The accelerometer is configured to a sampling rate of 200Hz with high performance mode and continuous update.
/// The full scale of the accelerometer (+-16g) is used.
///  
/// The acceleration value for each axis is read from the sensor and converted to a floating point value in gs.
///
/// Data sheet: https://www.st.com/resource/en/datasheet/lis2ds12.pdf
/// Application notes: https://www.st.com/resource/en/application_note/an4748-lis2ds12-alwayson-3axis-accelerometer-stmicroelectronics.pdf
pub struct Accelerometer<'a, T: HypedI2c + 'a> {
    i2c: &'a mut T,
    device_address: u8,
    calculate_bounds: fn(AccelerationValues) -> SensorValueRange<AccelerationValues>,
}

impl<'a, T: HypedI2c> Accelerometer<'a, T> {
    /// Create a new instance of the accelerometer and attempt to configure it
    pub fn new(
        i2c: &'a mut T,
        device_address: AccelerometerAddresses,
    ) -> Result<Self, AccelerometerError> {
        Self::new_with_bounds(i2c, device_address, default_calculate_bounds)
    }

    pub fn new_with_bounds(
        i2c: &'a mut T,
        device_address: AccelerometerAddresses,
        calculate_bounds: fn(AccelerationValues) -> SensorValueRange<AccelerationValues>,
    ) -> Result<Self, AccelerometerError> {
        let device_address = device_address as u8;

        i2c_write_or_err!(
            i2c,
            device_address,
            LIS2DS12_CTRL1_ADDRESS,
            LIS2DS12_CTRL1_VALUE,
            AccelerometerError
        );
        i2c_write_or_err!(
            i2c,
            device_address,
            LIS2DS12_CTRL2_ADDRESS,
            LIS2DS12_CTRL2_VALUE,
            AccelerometerError
        );
        i2c_write_or_err!(
            i2c,
            device_address,
            LIS2DS12_FIFO_CTRL_ADDRESS,
            LIS2DS12_FIFO_CTRL_VALUE,
            AccelerometerError
        );

        // Return Self only if all values are written successfully
        Ok(Self {
            i2c,
            device_address,
            calculate_bounds,
        })
    }

    /// Read the acceleration for each axis and return them as floating point values in gs.
    pub fn read(&mut self) -> Option<SensorValueRange<AccelerationValues>> {
        // Read the low and high bytes of the acceleration and combine them to get the acceleration for each axis
        let x_low_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_X_L)?;
        let x_high_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_X_H)?;
        let y_low_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Y_L)?;
        let y_high_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Y_H)?;
        let z_low_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Z_L)?;
        let z_high_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Z_H)?;

        let mut x_combined = ((x_high_byte as u16) << 8 | x_low_byte as u16) as f32;
        let mut y_combined = ((y_high_byte as u16) << 8 | y_low_byte as u16) as f32;
        let mut z_combined = ((z_high_byte as u16) << 8 | z_low_byte as u16) as f32;

        // Convert accelerations to to negative values if necessary.
        if x_combined >= TWO_POWER_15 {
            x_combined -= TWO_POWER_16;
        }
        if y_combined >= TWO_POWER_15 {
            y_combined -= TWO_POWER_16;
        }
        if z_combined >= TWO_POWER_15 {
            z_combined -= TWO_POWER_16;
        }
        let x = x_combined * LIS2DS12_ACCEL_SCALING_FACTOR;
        let y = y_combined * LIS2DS12_ACCEL_SCALING_FACTOR;
        let z = z_combined * LIS2DS12_ACCEL_SCALING_FACTOR;

        Some((self.calculate_bounds)(AccelerationValues { x, y, z }))
    }

    pub fn check_status(&mut self) -> Status {
        match self.i2c.read_byte(self.device_address, LIS2DS12_STATUS) {
            Some(byte) => Status::from_byte(byte),
            None => Status::Unknown,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AccelerationValues {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub enum AccelerometerAddresses {
    Address1d = 0x1D,
    Address1e = 0x1E,
}

#[derive(Debug)]
pub enum AccelerometerError {
    I2cError(I2cError),
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    DataNotReady,
    Unknown,
}

impl Status {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            LIS2DS12_DATA_NOT_READY => Self::DataNotReady,
            _ => Self::Ok,
        }
    }
}

/// Default calculation of bounds for the accelerometer, if no bounds function is provided.
/// The bounds are set to:
/// Safe: Between -6g and +6g
/// Warning: -8g to -6g and +6g to +8g
/// Critical: Below -8g and above +8g
pub fn default_calculate_bounds(
    values: AccelerationValues,
) -> SensorValueRange<AccelerationValues> {
    let mut values_iter = [values.x, values.y, values.z].into_iter(); // there's probably a better way of doing this
    if values_iter.any(|i| i >= 8000.0 || i <= -8000.0) {
        SensorValueRange::Critical(values)
    } else if values_iter.any(|i| i >= 6000.0 || i <= -6000.0) {
        SensorValueRange::Warning(values)
    } else {
        SensorValueRange::Safe(values)
    }
}

// Registers for the LIS2DS12 accelerometer
const LIS2DS12_CTRL1_ADDRESS: u8 = 0x20;
const LIS2DS12_CTRL2_ADDRESS: u8 = 0x21;
const LIS2DS12_FIFO_CTRL_ADDRESS: u8 = 0x25;

const LIS2DS12_OUT_X_L: u8 = 0x28;
const LIS2DS12_OUT_X_H: u8 = 0x29;
const LIS2DS12_OUT_Y_L: u8 = 0x2A;
const LIS2DS12_OUT_Y_H: u8 = 0x2B;
const LIS2DS12_OUT_Z_L: u8 = 0x2C;
const LIS2DS12_OUT_Z_H: u8 = 0x2D;

// Scaling factor to convert accelerations into gs
const LIS2DS12_ACCEL_SCALING_FACTOR: f32 = 0.488;

// For handling 2's complement values
const TWO_POWER_15: f32 = 32768.0;
const TWO_POWER_16: f32 = 65536.0;

const LIS2DS12_STATUS: u8 = 0x27;
// Values to check the status of the accelerometer
const LIS2DS12_DATA_NOT_READY: u8 = 0x00;

// Values written to control registers (may need to change later)
const LIS2DS12_CTRL1_VALUE: u8 = 0x54; // 200Hz, high performance, continuous, +-16g
const LIS2DS12_CTRL2_VALUE: u8 = 0x0;
const LIS2DS12_FIFO_CTRL_VALUE: u8 = 0x30;

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
        let _ = Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d);
        assert_eq!(
            i2c.get_writes().get(&(
                AccelerometerAddresses::Address1d as u8,
                LIS2DS12_CTRL1_ADDRESS
            )),
            Some(&Some(LIS2DS12_CTRL1_VALUE))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                AccelerometerAddresses::Address1d as u8,
                LIS2DS12_CTRL2_ADDRESS
            )),
            Some(&Some(LIS2DS12_CTRL2_VALUE))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                AccelerometerAddresses::Address1d as u8,
                LIS2DS12_FIFO_CTRL_ADDRESS
            )),
            Some(&Some(LIS2DS12_FIFO_CTRL_VALUE))
        );
    }

    #[test]
    fn test_accel_read_zero() {
        let mut i2c_values = FnvIndexMap::new();

        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0x00),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }))
        );
    }

    #[test]
    fn test_accel_read_x() {
        let mut i2c_values = FnvIndexMap::new();

        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0x09),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0xc4),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0x00),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: 1220.0,
                y: 0.0,
                z: 0.0
            }))
        );
    }

    #[test]
    fn test_accel_read_minus_x() {
        let mut i2c_values = FnvIndexMap::new();

        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0xf6),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0x3c),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0x00),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: -1220.0,
                y: 0.0,
                z: 0.0
            }))
        );
    }

    #[test]
    fn test_accel_read_y() {
        let mut i2c_values = FnvIndexMap::new();

        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0x09),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0xc4),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0x00),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: 0.0,
                y: 1220.0,
                z: 0.0
            }))
        );
    }

    #[test]
    fn test_accel_read_minus_y() {
        let mut i2c_values = FnvIndexMap::new();

        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0xf6),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0x3c),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0x00),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: 0.0,
                y: -1220.0,
                z: 0.0
            }))
        );
    }

    #[test]
    fn test_accel_read_z() {
        let mut i2c_values = FnvIndexMap::new();

        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0x09),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0xc4),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: 0.0,
                y: 0.0,
                z: 1220.0
            }))
        );
    }

    #[test]
    fn test_accel_read_minus_z() {
        let mut i2c_values = FnvIndexMap::new();

        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0xf6),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0x3c),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: 0.0,
                y: 0.0,
                z: -1220.0
            }))
        );
    }

    #[test]
    fn test_accel_read_combined() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_H),
            Some(0x00),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_X_L),
            Some(0xfa),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_H),
            Some(0x01),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Y_L),
            Some(0xf4),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_H),
            Some(0xfc),
        );
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_OUT_Z_L),
            Some(0x18),
        );

        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(
            accelerometer.read(),
            Some(SensorValueRange::Safe(AccelerationValues {
                x: 122.0,
                y: 244.0,
                z: -488.0
            }))
        );
    }

    #[test]
    fn test_accel_status_data_not_ready() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (AccelerometerAddresses::Address1d as u8, LIS2DS12_STATUS),
            Some(LIS2DS12_DATA_NOT_READY),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut accelerometer =
            Accelerometer::new(&mut i2c, AccelerometerAddresses::Address1d).unwrap();
        assert_eq!(accelerometer.check_status(), Status::DataNotReady);
    }
}
