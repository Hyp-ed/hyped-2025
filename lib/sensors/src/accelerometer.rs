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
