use crate::{HypedI2c, I2cError};

pub struct I2cMux<T: HypedI2c> {
    i2c: T,
    mux_address: u8,
    channel: u8,
}

#[derive(Debug)]
pub enum I2cMuxError {
    InvalidChannel,
}

impl<T: HypedI2c> I2cMux<T> {
    pub fn new(i2c: T, channel: u8, mux_address: u8) -> Result<Self, I2cMuxError> {
        // Check that the channel is valid
        if channel >= MAX_MUX_CHANNELS {
            return Err(I2cMuxError::InvalidChannel);
        }
        Ok(Self {
            i2c,
            channel,
            mux_address,
        })
    }

    /// Selects the channel on the multiplexer
    fn select_channel(&mut self) -> Result<(), I2cError> {
        match self.i2c.write_byte(self.mux_address, 1 << self.channel) {
            Ok(_) => Ok(()),
            Err(e) => Err(e as I2cError),
        }
    }
}

impl<T: HypedI2c> HypedI2c for I2cMux<T> {
    fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
        match self.select_channel() {
            Ok(_) => self.i2c.read_byte(device_address, register_address),
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

    fn write_byte(&mut self, device_address: u8, data: u8) -> Result<(), I2cError> {
        match self.select_channel() {
            Ok(_) => self.i2c.write_byte(device_address, data),

            Err(e) => Err(e as I2cError),
        }
    }
}

pub const DEFAULT_MUX_ADDRESS: u8 = 0x70;
const MAX_MUX_CHANNELS: u8 = 8;
