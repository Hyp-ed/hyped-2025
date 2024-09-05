use hyped_core::types::Status;
use hyped_io::i2c::HypedI2c;

pub enum TemperatureAddress {
    X7f = 0x7F,
    X38 = 0x38,
    X3c = 0x3C,
    X3e = 0x3E,
}

pub struct Temperature {
    device_address: TemperatureAddress,
}

impl Temperature {
    pub fn new(device_address: TemperatureAddress) -> Self {
        Temperature { device_address }
    }

    /// Read the temperature from the sensor
    pub fn read(&self) -> f32 {
        todo!()
    }

    /// Checks if the temperature sensor is ready to be read.
    pub fn check_status(&self) -> Status {
        todo!()
    }
}
