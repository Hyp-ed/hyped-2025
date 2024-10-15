/// I2C trait used to abstract the I2C peripheral

#[derive(Debug)]
pub enum I2cError {
    // Error codes nased https://docs.embassy.dev/embassy-stm32/git/stm32g031c8/i2c/enum.Error.html
    Bus,
    Arbitration,
    Nack,
    Timeout,
    Crc,
    Overrun,
    ZeroLengthTransfer,
    Unknown,
}

pub trait HypedI2c {
    fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8>;
    fn write_byte_to_register(
        &mut self,
        device_address: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), I2cError>;
}
