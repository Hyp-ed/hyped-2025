use crate::HypedAdc;
use hyped_i2c::{HypedI2c, I2cError};

/// Implements logic to read a specific channel on the ADC Mux
///
/// Data sheet: https://www.ti.com/lit/ds/symlink/adc128d818.pdf
pub struct ADCMuxChannel<T: HypedI2c> {
    i2c: T,
    device_address: u8,
    channel: AdcChannelAddress,
}

impl<T: HypedI2c> ADCMuxChannel<T> {
    pub fn new(
        i2c: T,
        device_address: u8,
        channel: AdcChannelAddress,
    ) -> Result<Self, AdcMuxError> {
        Ok(Self {
            i2c,
            device_address,
            channel,
        })
    }
}

impl<T: HypedI2c> HypedAdc for ADCMuxChannel<T> {
    /// Read a value from the ADC Mux channel
    fn read_value(&mut self) -> u16 {
        self.i2c
            .read_byte(self.device_address, self.channel as u8)
            .unwrap() as u16
    }
    /// Get the resolution of the ADC Mux
    fn get_resolution(&self) -> u16 {
        // 12-bit resolution
        4096
    }
}

/// Implements logic to read a specified number of channel registers on the ADC Mux
/// in one go
///
/// Data sheet: https://www.ti.com/lit/ds/symlink/adc128d818.pdf
pub struct ADCMux<T: HypedI2c> {
    i2c: T,
    device_address: u8,
}

impl<T: HypedI2c> ADCMux<T> {
    pub fn new(i2c: T, device_address: u8) -> Result<Self, AdcMuxError> {
        Ok(Self {
            i2c,
            device_address,
        })
    }

    /// Read all ADC Mux channels
    pub fn read_data(&mut self) -> Result<[Option<u8>; MAX_MUX_CHANNELS], AdcMuxError> {
        let mut mux_data = [None; MAX_MUX_CHANNELS];

        for i in 0..MAX_MUX_CHANNELS {
            let channel_address = AdcChannelAddress::AdcChannel0 as u8 + i as u8;
            mux_data[i] = self.i2c.read_byte(self.device_address, channel_address);
        }

        Ok(mux_data)
    }
}

pub const ADC_MUX_ADDRESS: u8 = 0x1D;
pub const MAX_MUX_CHANNELS: usize = 8;

/// ADC Mux channel addresses
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AdcChannelAddress {
    AdcChannel0 = 0x20,
    AdcChannel1 = 0x21,
    AdcChannel2 = 0x22,
    AdcChannel3 = 0x23,
    AdcChannel4 = 0x24,
    AdcChannel5 = 0x25,
    AdcChannel6 = 0x26,
    AdcChannel7 = 0x27,
}

/// Possible errors that can occur when interacting with the ADC Mux
#[derive(Debug)]
pub enum AdcMuxError {
    I2cError(I2cError),
}
