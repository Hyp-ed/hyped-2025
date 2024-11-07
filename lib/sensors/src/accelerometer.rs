use hyped_io::i2c::{HypedI2c, I2cError};

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
pub struct Accelerometer<T: HypedI2c> {
    i2c: T,
    device_address: u8,
}

impl<T: HypedI2c> Accelerometer<T> {
    /// Create a new instance of the accelerometer and attempt to configure it
    pub fn new(mut i2c: T, device_address: AccelerometerAddresses) -> Result<Self, AccelerometerError> {
        let device_address = device_address as u8;
        if let Err(e) = i2c.write_byte_to_register(device_address, LIS2DS12_CTRL1_ADDRESS, LIS2DS12_CTRL1_VALUE) {
            return Err(AccelerometerError::I2cError(e));
        }
        if let Err(e) = i2c.write_byte_to_register(device_address, LIS2DS12_CTRL2_ADDRESS, LIS2DS12_CTRL2_VALUE) {
            return Err(AccelerometerError::I2cError(e));
        }
        if let Err(e) = i2c.write_byte_to_register(device_address, LIS2DS12_FIFO_CTRL_ADDRESS, LIS2DS12_FIFO_CTRL_VALUE) {
            return Err(AccelerometerError::I2cError(e));
        }
        // Return Self only if all values are written successfully
        Ok(Self{i2c, device_address})
    }

    /// Read the acceleration for each axis and return them as floating point values in gs.
    pub fn read(&mut self) -> Option<AccelerationValues> {

        // Read the low and high bytes of the acceleration and combine them to get the acceleration for each axis
        let x_low_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_X_L)?;
        let x_high_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_X_H)?;
        let y_low_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Y_L)?;
        let y_high_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Y_H)?;
        let z_low_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Z_L)?;
        let z_high_byte = self.i2c.read_byte(self.device_address, LIS2DS12_OUT_Z_H)?;
    
        let x_combined = ((x_high_byte as u16) << 8 | x_low_byte as u16) as f32;
        let y_combined = ((y_high_byte as u16) << 8 | y_low_byte as u16) as f32;
        let z_combined = ((z_high_byte as u16) << 8 | z_low_byte as u16) as f32;

        let x = x_combined * LIS2DS12_ACCEL_SCALING_FACTOR;
        let y = y_combined * LIS2DS12_ACCEL_SCALING_FACTOR;
        let z = z_combined * LIS2DS12_ACCEL_SCALING_FACTOR;

        Some(AccelerationValues { x, y, z })
    }

    pub fn check_status(&mut self) -> Status {
        match self.i2c.read_byte(self.device_address, LIS2DS12_STATUS) {
            Some(byte) => Status::from_byte(byte),
            None => Status::Unknown,
        }
    }

}

#[allow(dead_code)]
pub struct AccelerationValues {
    x: f32,
    y: f32,
    z: f32,
}

pub enum AccelerometerAddresses {
    Address1d = 0x1D,
    Address1e = 0x1E,
}

pub enum AccelerometerError {
    I2cError(I2cError),
}

pub enum Status {
    Ok,
    DataNotReady,
    Unknown
}

impl Status {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            LIS2DS12_DATA_NOT_READY => Self::DataNotReady,
            _ => Self::Ok,
        }
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

const LIS2DS12_STATUS: u8 = 0x27;
// Values to check the status of the accelerometer
const LIS2DS12_DATA_NOT_READY: u8 = 0x00;

// Values written to control registers (may need to change later)
const LIS2DS12_CTRL1_VALUE: u8 = 0x54; // 200Hz, high performance, continuous, +-16g
const LIS2DS12_CTRL2_VALUE: u8 = 0x0;
const LIS2DS12_FIFO_CTRL_VALUE: u8 = 0x30;
