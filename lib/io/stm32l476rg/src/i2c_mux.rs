use hyped_io::i2c::HypedI2c;

use crate::i2c::Stm32l476rgI2c;

pub struct Stm32l476rgI2cMux<'a> {
    i2c: Stm32l476rgI2c<'a>,
    mux_address: u8,
    channel: u8,
}

impl<'d> Stm32l476rgI2cMux<'d> {
    pub fn new(i2c: Stm32l476rgI2c<'d>, channel: u8, mux_address: u8) -> Result<Self, ()> {
        // Check that the channel is valid
        if channel >= MAX_MUX_CHANNELS {
            return Err(());
        }
        Ok(Self {
            i2c,
            channel,
            mux_address,
        })
    }

    pub fn select_channel(&mut self) -> Result<(), ()> {
        match self.i2c.write_byte(self.mux_address, 1 << self.channel) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

impl<'d> HypedI2c for Stm32l476rgI2cMux<'d> {
    fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
        match self.select_channel() {
            Ok(_) => self.i2c.read_byte(device_address, register_address),
            Err(_) => None,
        }
    }

    fn write_byte_to_register(
        &mut self,
        device_addres: u8,
        register_address: u8,
        data: u8,
    ) -> Result<(), ()> {
        match self.select_channel() {
            Ok(_) => self
                .i2c
                .write_byte_to_register(device_addres, register_address, data),
            Err(_) => Err(()),
        }
    }

    fn write_byte(&mut self, device_address: u8, data: u8) -> Result<(), ()> {
        match self.select_channel() {
            Ok(_) => self.i2c.write_byte(device_address, data),
            Err(_) => Err(()),
        }
    }
}

pub const DEFAULT_MUX_ADDRESS: u8 = 0x70;
const MAX_MUX_CHANNELS: u8 = 8;
