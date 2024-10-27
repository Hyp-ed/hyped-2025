use hyped_io::i2c::{HypedI2c, I2cError};

pub struct LedDriver<T: HypedI2c> {
    i2c: T,
    device_address: u8,
}

impl<T: HypedI2c> LedDriver<T> {
    /// Create new instance of LED driver and attempt to configure
    pub fn initialise(mut i2c: T, device_address: ledConfigRegister) -> Result<Self, ledDriverError> {
        
    }
}



const defaultLedDriverAddress: u8 = 0x30;

// Registers for LED0 (maybe for ^ as well?)
const ledConfigRegister: u8 = 0x02;
const brightnessRegisterBase: u8 = 0x08;
const colourRegisterBase: u8 = 0x14;
const resetRegister: u8 = 0x38;