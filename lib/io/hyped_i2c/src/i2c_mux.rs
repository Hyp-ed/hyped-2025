use crate::{HypedI2c, I2cError};

#[derive(Debug)]
pub enum I2cMuxError {
    InvalidChannel,
    FailedToSelectChannel,
}

/// A struct that represents an I2C multiplexer. (TCA9548A Low-Voltage 8-channel I2C Switch with Reset.)
///
/// The I2cMux struct is a wrapper around an I2c struct that adds the ability to select a channel on
/// a multiplexer before performing an I2C operation. Multiplexers are used to allow multiple I2C
/// devices to share the same I2C bus by selecting which device is connected to the bus.
///
/// The I2cMux struct implements the HypedI2c trait, which allows it to be used in place of an I2c
/// struct in any code that uses the HypedI2c trait.
///
/// Data sheet: https://www.ti.com/lit/ds/symlink/tca9548a.pdf
pub struct I2cMux<T: HypedI2c> {
    i2c: T,
    mux_address: u8,
    channel: u8,
}

impl<T: HypedI2c> I2cMux<T> {
    pub fn new(i2c: T, channel: u8, mux_address: u8) -> Result<Self, I2cMuxError> {
        // Check that the channel is valid (channels start at 0)
        if channel >= MAX_MUX_CHANNELS {
            return Err(I2cMuxError::InvalidChannel);
        }
        let mut mux = Self {
            i2c,
            channel,
            mux_address,
        };
        match mux.select_channel() {
            Ok(_) => {}
            Err(_) => return Err(I2cMuxError::FailedToSelectChannel),
        }
        Ok(mux)
    }

    /// Selects the channel on the multiplexer by writing the channel number
    fn select_channel(&mut self) -> Result<(), I2cError> {
        self.i2c.write_byte(self.mux_address, 1 << self.channel)
    }
}

/// The implementations of the HypedI2c trait for the I2cMux struct simply select the channel on the
/// multiplexer before calling the corresponding method on the inner I2c struct.
impl<T: HypedI2c> HypedI2c for I2cMux<T> {
    fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
        match self.select_channel() {
            Ok(_) => self.i2c.read_byte(device_address, register_address),
            Err(_) => None,
        }
    }
    fn read_byte_16(&mut self, device_address: u8, register_address: u16) -> Option<u8> {
        match self.select_channel() {
            Ok(_) => self.i2c.read_byte_16(device_address, register_address),
            Err(_) => None,
        }
    }

    fn write_byte_to_register(
        &mut self,
        device_address: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), I2cError> {
        match self.select_channel() {
            Ok(_) => self
                .i2c
                .write_byte_to_register(device_address, register_address, data),
            Err(e) => Err(e as I2cError),
        }
    }

    fn write_byte_to_register_16(
        &mut self,
        device_address: u8,
        register_address: u16,
        data: u8,
    ) -> Result<(), I2cError> {
        match self.select_channel() {
            Ok(_) => self
                .i2c
                .write_byte_to_register_16(device_address, register_address, data),
            Err(e) => Err(e as I2cError),
        }
    }

    fn write_byte(&mut self, device_address: u8, data: u8) -> Result<(), I2cError> {
        match self.select_channel() {
            Ok(_) => self.i2c.write_byte(device_address, data),

            Err(e) => Err(e as I2cError),
        }
    }
}

pub const DEFAULT_MUX_ADDRESS: u8 = 0x70;
const MAX_MUX_CHANNELS: u8 = 8;
