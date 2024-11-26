#![no_std]

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
    fn write_byte_to_register(
        &mut self,
        device_address: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), I2cError>;
}

pub mod mock_i2c {
    use core::cell::RefCell;
    use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
    use heapless::FnvIndexMap;

    /// A fixed-size map of I2C values, indexed by device address and register address
    type I2cValues = FnvIndexMap<(u8, u8), Option<u8>, 16>;

    /// A mock I2C instance which can be used for testing
    pub struct MockI2c<'a> {
        values: &'a Mutex<CriticalSectionRawMutex, RefCell<I2cValues>>,
        writes: I2cValues,
    }

    impl crate::HypedI2c for MockI2c {
        /// Reads a byte by looking up the device address and register address in the map
        fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
            self.values.lock(|values| {
                values
                    .borrow()
                    .get(&(device_address, register_address))
                    .copied()
                    .unwrap()
            })
        }

        /// Writes a byte to the map so that it can be read later to check the value
        fn write_byte_to_register(
            &mut self,
            device_address: u8,
            register_address: u8,
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

    impl MockI2c<'_> {
        pub fn new(values: &Mutex<CriticalSectionRawMutex, RefCell<I2cValues>>) -> MockI2c {
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
