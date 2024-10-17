// I2C trait used to abstract the I2C peripheral
pub trait HypedI2c {
    fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8>;
    fn write_byte_to_register(
        &mut self,
        device_address: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), ()>;
}