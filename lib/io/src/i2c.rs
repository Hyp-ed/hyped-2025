/// Error codes from: https://docs.embassy.dev/embassy-stm32/git/stm32g031c8/i2c/enum.Error.html
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
    fn write_byte_to_register(
        &mut self,
        device_address: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), I2cError>;
}

pub mod mock_i2c {
    use heapless::FnvIndexMap;

    /// A fixed-size map of I2C values, indexed by device address and register address
    type I2cValues = FnvIndexMap<(u8, u8), u8, 16>;

    /// A mock I2C instance which can be used for testing
    pub struct MockI2c {
        values: I2cValues,
    }

    impl crate::i2c::HypedI2c for MockI2c {
        /// Reads a byte by looking up the device address and register address in the map
        fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
            self.values
                .get(&(device_address, register_address))
                .copied()
        }

        /// Always succeeds, but does nothing
        fn write_byte_to_register(
            &mut self,
            _device_address: u8,
            _register_address: u8,
            _data: u8,
        ) -> Result<(), super::I2cError> {
            Ok(())
        }
    }

    impl MockI2c {
        pub fn new(values: I2cValues) -> MockI2c {
            MockI2c { values }
        }

        /// Inserts a value into the map. If the value already exists, it is overwritten.
        pub fn insert(&mut self, device_address: u8, register_address: u8, value: u8) {
            let _ = self
                .values
                .insert((device_address, register_address), value);
        }
    }
}
