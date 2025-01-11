/// I2C errors that can occur
/// From: https://docs.embassy.dev/embassy-stm32/git/stm32g031c8/i2c/enum.Error.html
#[derive(Debug)]
pub enum I2cError {
    Bus,
    Arbitration,
    Nack,
    Timeout,
    Crc,
    Overrun,
    ZeroLengthTransfer,
    Unknown,
}

/// I2C trait used to abstract the I2C peripheral
pub trait HypedI2c {
    fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8>;
    fn read_byte_16(&mut self, device_address: u8, register_address: u16) -> Option<u8>;
    fn write_byte_to_register(
        &mut self,
        device_address: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), I2cError>;
    fn write_byte_to_register_16(
        &mut self,
        device_address: u8,
        register_address: u16,
        data: u8,
    ) -> Result<(), I2cError>;
}

pub mod mock_i2c {
    use heapless::FnvIndexMap;

    /// A fixed-size map of I2C values, indexed by device address and register address
    type I2cValues = FnvIndexMap<(u8, u16), Option<u8>, 64>;

    /// A mock I2C instance which can be used for testing
    pub struct MockI2c {
        values: I2cValues,
        writes: I2cValues,
    }

    impl crate::i2c::HypedI2c for MockI2c {
        /// Reads a byte by looking up the device address and register address in the map
        fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
            self.values
                .get(&(device_address, register_address.into()))
                .copied()
                .unwrap()
        }

        /// Reads a byte by looking up the device address and register address in the map
        fn read_byte_16(&mut self, device_address: u8, register_address: u16) -> Option<u8> {
            self.values
                .get(&(device_address, register_address))
                .copied()
                .unwrap()
        }

        /// Writes a byte to the map (with 8-bit register address) so that it can be read later to check the value
        fn write_byte_to_register(
            &mut self,
            device_address: u8,
            register_address: u8,
            data: u8,
        ) -> Result<(), super::I2cError> {
            match self
                .writes
                .insert((device_address, register_address.into()), Some(data))
            {
                Ok(_) => Ok(()),
                Err(_) => Err(super::I2cError::Unknown),
            }
        }

        /// Writes a byte to the map (with 16-bit register address) so that it can be read later to check the value
        fn write_byte_to_register_16(
            &mut self,
            device_address: u8,
            register_address: u16,
            data: u8,
        ) -> Result<(), super::I2cError> {
            match self
                .writes
                .insert((device_address, register_address), Some(data))
            {
                Ok(_) => Ok(()),
                Err(_) => Err(super::I2cError::Unknown),
            }
        }
    }

    impl MockI2c {
        pub fn new(values: I2cValues) -> MockI2c {
            MockI2c {
                values,
                writes: I2cValues::new(),
            }
        }

        /// Returns a reference to the I2C values map
        pub fn get_writes(&self) -> &I2cValues {
            &self.writes
        }
    }
}
