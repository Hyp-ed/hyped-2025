use hyped_io::i2c::HypedI2c;

pub struct TimeOfFlight<T: HypedI2c> {
    i2c: T,
    device_address: u8,
}

impl<T: HypedI2c> TimeOfFlight<T> {
    // Create a new instance of time of flight sensor, configure
    pub fn new(mut i2c: T, device_address: ToFAddresses) -> Result<Self, ()> {
        let device_address = device_address as u8;
        // Set up the ToF sensor, sending config settings and setting mode to the appropriate registers

    }
}

pub enum ToFAddresses {
    Address29 = 0x29,
}