use crate::i2c::I2C;

pub struct EmbassyI2C {}

impl I2C for EmbassyI2C {
    fn read_byte(device_address: u8, register_addresrs: u8) -> u8 {
        todo!()
    }
    fn write_byte(device_address: u8, data: u8) -> Result<(), ()> {
        todo!()
    }
    fn write_byte_to_register(device_addres: u8, register_address: u8, data: u8) -> Result<(), ()> {
        todo!()
    }
}

impl EmbassyI2C {
    pub fn new() -> Self {
        todo!()
    }
}
