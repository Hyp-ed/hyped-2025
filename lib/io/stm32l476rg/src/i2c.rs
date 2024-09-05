use embassy_stm32::{i2c::I2c, mode::Blocking};
use hyped_io::i2c::HypedI2c;

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
        device_addres: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), ()> {
        let result = self
            .i2c
            .blocking_write(device_addres, [register_address, data].as_ref());
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

impl<'d> Stm32l476rgI2c<'d> {
    /// Create a new instance of our I2C implementation for the STM32L476RG
    pub fn new(i2c: I2c<'d, Blocking>) -> Self {
        Self { i2c }
    }
}
