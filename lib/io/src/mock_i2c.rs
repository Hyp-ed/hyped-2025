use crate::i2c::I2C;

pub struct MockI2C {}

impl I2C for MockI2C {
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

impl MockI2C {
    pub fn new() -> Self {
        todo!()
    }
}
