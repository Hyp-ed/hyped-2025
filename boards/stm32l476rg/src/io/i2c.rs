use embassy_stm32::{i2c::I2c, mode::Blocking};
use hyped_io::i2c::{HypedI2c, I2cError};

pub struct Stm32l476rgI2c<'d> {
    i2c: I2c<'d, Blocking>,
}

impl<'d> HypedI2c for Stm32l476rgI2c<'d> {
    /// Read a byte from a register on a device
    fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
        let mut read = [0];
        let result =
            self.i2c
                .blocking_write_read(device_address, [register_address].as_ref(), &mut read);
        match result {
            Ok(_) => Some(read[0]),
            Err(_) => None,
        }
    }

    /// Write a byte to a register on a device
    fn write_byte_to_register(
        &mut self,
        device_address: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), I2cError> {
        let result = self
            .i2c
            .blocking_write(device_address, [register_address, data].as_ref());
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(match e {
                embassy_stm32::i2c::Error::Bus => I2cError::Bus,
                embassy_stm32::i2c::Error::Arbitration => I2cError::Arbitration,
                embassy_stm32::i2c::Error::Nack => I2cError::Nack,
                embassy_stm32::i2c::Error::Timeout => I2cError::Timeout,
                embassy_stm32::i2c::Error::Crc => I2cError::Crc,
                embassy_stm32::i2c::Error::Overrun => I2cError::Overrun,
                embassy_stm32::i2c::Error::ZeroLengthTransfer => I2cError::ZeroLengthTransfer,
            }),
        }
    }

    /// Write a byte to a device
    fn write_byte(&mut self, device_address: u8, data: u8) -> Result<(), I2cError> {
        let result = self.i2c.blocking_write(device_address, [data].as_ref());
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(match e {
                embassy_stm32::i2c::Error::Bus => I2cError::Bus,
                embassy_stm32::i2c::Error::Arbitration => I2cError::Arbitration,
                embassy_stm32::i2c::Error::Nack => I2cError::Nack,
                embassy_stm32::i2c::Error::Timeout => I2cError::Timeout,
                embassy_stm32::i2c::Error::Crc => I2cError::Crc,
                embassy_stm32::i2c::Error::Overrun => I2cError::Overrun,
                embassy_stm32::i2c::Error::ZeroLengthTransfer => I2cError::ZeroLengthTransfer,
            }),
        }
    }
}

impl<'d> Stm32l476rgI2c<'d> {
    /// Create a new instance of our I2C implementation for the STM32L476RG
    pub fn new(i2c: I2c<'d, Blocking>) -> Self {
        Self { i2c }
    }
}
