use hyped_io::i2c::{HypedI2c, I2cError};

/// ADCMux implements logic to read a specified number of channel registers on the ADC Mux
/// and report any failures
///
/// Data sheet: https://www.ti.com/lit/ds/symlink/adc128d818.pdf
pub struct ADCMux<T: HypedI2c> {
    i2c: T,
    device_address: u8,
    channels: [Option<u8>; MAX_MUX_CHANNELS],
    num_channels: usize,
    max_num_faulty_channels: u8,
}

impl<T: HypedI2c> ADCMux<T> {
    /// Create a new instance of the ADC mux sensor and configure it
    /// num_channels: number of channels to use (up to 8 max)
    pub fn new(mut i2c: T, device_address: u8, num_channels: usize) -> Result<Self, MuxError> {
        if num_channels > MAX_MUX_CHANNELS {
            return Err(MuxError::TooManyChannels);
        }

        let max_num_faulty_channels = (FAILURE_THRESHOLD * num_channels as f32).round() as u8;

        Ok(Self {
            i2c,
            device_address,
            channels: [None; MAX_MUX_CHANNELS],
            num_channels,
            max_num_faulty_channels,
        })
    }

    /// Read ADC Mux channels and report any failures
    pub fn read_data(&mut self) -> Result<[Option<u8>; MAX_MUX_CHANNELS], MuxError> {
        let mut mux_data = [None; MAX_MUX_CHANNELS];
        let mut num_faulty_channels = 0;

        for i in 0..self.num_channels {
            let channel_address = CHANNEL_ADDRESSES[i];
            match self.i2c.read_byte(self.device_address, channel_address) {
                Some(data) => mux_data[i] = Some(data),
                None => {
                    num_faulty_channels += 1;
                    if num_faulty_channels > self.max_num_faulty_channels {
                        return Err(MuxError::TooManyFaultyChannels);
                    }
                }
            }
        }

        Ok(mux_data)
    }
}

/// ADC Mux addresses
const ADC_MUX_ADDRESS: u8 = 0x1D;
const CHANNEL_ADDRESSES: [u8; MAX_MUX_CHANNELS] = [
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
];

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