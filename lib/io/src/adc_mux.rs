use hyped_io::i2c::{HypedI2c, I2cError};

/// ADCMux implements logic to read a specified number of channel registers on the ADC Mux
/// and report any failures
///
/// Data sheet: https://www.ti.com/lit/ds/symlink/adc128d818.pdf
pub struct ADCMuxChannel<T: HypedI2c> {
    i2c: T,
    device_address: u8,
    channel: AdcChannelAddress,
}


impl<T: HypedI2c> ADCMuxChannel<T> {
    /// Create a new instance of the ADC mux sensor and configure it
    /// num_channels: number of channels to use (up to 8 max)
    pub fn new(mut i2c: T, device_address: u8, channel: AdcChannelAddress) -> Result<Self, MuxError> {
        Ok(Self {
            i2c,
            device_address,
            channel,
        })
    }
}

impl HypedAdc for ADCMuxChannel<I2c> {
    /// Read a value from the ADC Mux channel
    fn read_value(&mut self) -> u16 {
        self.i2c.read_byte(self.device_address, ADC_MUX_ADDRESS).unwrap() as u16
    }
}

/// ADC Mux addresses
const ADC_MUX_ADDRESS: u8 = 0x1D;
pub enum AdcChannelAddress {
    AdcChannel20 = 0x20, 
    AdcChannel21 = 0x21, 
    AdcChannel22 = 0x22, 
    AdcChannel23 = 0x23, 
    AdcChannel24 = 0x24, 
    AdcChannel25 = 0x25, 
    AdcChannel26 = 0x26, 
    AdcChannel27 = 0x27,
}

/// Possible errors that can occur when interacting with the ADC Mux
#[derive(Debug)]
pub enum MuxError {
    I2cError(I2cError),
    TooManyFaultyChannels,
    TooManyChannels,
}

// Values for error checking
const FAILURE_THRESHOLD: f32 = 0.25;
const MAX_MUX_CHANNELS: usize = 8;