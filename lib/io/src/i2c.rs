use embassy_stm32::i2c::I2C;

pub trait I2C {
    fn read_byte(device_address: u8, register_addresrs: u8) -> u8;
    fn write_byte_to_register(device_addres: u8, register_address: u8, data: u8) -> Result<(), ()>;
    fn write_byte(device_address: u8, data: u8) -> Result<(), ()>;
}
