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
        let _ctrl_1_result = match i2c.write_byte_to_register(device_address, LIS2DS12_CTRL1_ADDRESS, LIS2DS12_CTRL1_VALUE) {
            Err(e) => return Err(AccelerometerError::I2cError(e)),
            Ok(p) =>  p
        };
        let _ctrl_2_result = match i2c.write_byte_to_register(device_address, LIS2DS12_CTRL2_ADDRESS, LIS2DS12_CTRL2_VALUE) {
            Err(e) => return Err(AccelerometerError::I2cError(e)),
            Ok(p) =>  p
        };
        let _fifo_ctrl_result = match i2c.write_byte_to_register(device_address, LIS2DS12_FIFO_CTRL_ADDRESS, LIS2DS12_FIFO_CTRL_VALUE) {
            Err(e) => return Err(AccelerometerError::I2cError(e)),
            Ok(p) =>  p
        };

        // Return Self only if all values are written successfully
        Ok(Self{i2c, device_address})
    }
}

pub enum AccelerometerAddresses {
    Address1d = 0x1D,
    Address1e = 0x1E,
}

pub enum AccelerometerError {
    I2cError(I2cError),
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

const LIS2DS12_STATUS: u8 = 0x27;

// Values written to control registers (may need to change later)
const LIS2DS12_CTRL1_VALUE: u8 = 0x54; // 200Hz, high performance, continuous, +-16g
const LIS2DS12_CTRL2_VALUE: u8 = 0x0;
const LIS2DS12_FIFO_CTRL_VALUE: u8 = 0x30;
