use crate::{
    adc::HypedAdc,
    i2c::{HypedI2c, I2cError},
};

/// ADCMux implements logic to read a channel on the ADC Mux and report any failures
///
/// Data sheet: https://www.ti.com/lit/ds/symlink/adc128d818.pdf
pub struct ADCMuxChannel<T: HypedI2c> {
    i2c: T,
    device_address: u8,
    channel: AdcChannelAddress,
}

impl<T: HypedI2c> ADCMuxChannel<T> {
    /// Create a new instance of the ADC mux sensor and configure it
    pub fn new(i2c: T, device_address: u8, channel: AdcChannelAddress) -> Result<Self, MuxError> {
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
}

pub const ADC_MUX_ADDRESS: u8 = 0x1D;

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
pub enum MuxError {
    I2cError(I2cError),
}
