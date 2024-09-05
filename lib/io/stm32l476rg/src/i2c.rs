use embassy_stm32::i2c::{I2c, Instance};
use hyped_io::i2c::HypedI2c;

pub struct Stm32l476rgGpioI2c<'d, T: Instance> {
    i2c: I2c<'d, T>,
}

impl<'d, T: Instance> HypedI2c for Stm32l476rgGpioI2c<'d, T> {
    fn read_byte(&self, register_address: u8) -> u8 {
        let mut buffer = [0];
        self.i2c.read(register_address, &buffer);
        buffer[0]
    }

    fn write_byte(&self, device_address: u8, data: u8) -> Result<(), ()> {
        todo!()
    }

    fn write_byte_to_register(
        &self,
        device_addres: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), ()> {
        todo!()
    }
}

impl<'d, T: Instance> Stm32l476rgGpioI2c<'d, T> {
    pub fn new(&mut i2c: I2c<'d, T>) -> Self {
        Self { i2c }
    }
}
