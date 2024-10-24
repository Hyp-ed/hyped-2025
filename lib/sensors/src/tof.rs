use hyped_io::i2c::{HypedI2c, I2cError};

///ToF implements the logic to read Time of Flight data from the VL6180V1 Time of Flight
///sensor using I2C peripheral provided by the Hyped I2c trait.
///
/// finish this write up later.
/// 
/// Data sheet:


pub struct TimeOfFlight<T: HypedI2c> {
    i2c: T,
    device_address: u8,
}

impl<T: HypedI2c> TimeOfFlight<T> {
    // Create a new instance of time of flight sensor, configure
    pub fn new(mut i2c: T, device_address: ToFAddresses) -> Result<Self, ToFError> {
        let device_address = device_address as u8;
        // Set up the ToF sensor, sending config settings and setting mode to the appropriate registers
        // need to set up loops / iterators to match i2c.write_byte_to_register to various addresses...
        for n in 0..30 { // private registers
            i2c.write_byte_to_register(device_address, PRIVATE_REGISTERS[n], PRIVATE_REGISTER_DATA[n]);
        }
        return Ok(Self {
            i2c,
            device_address,
        })  // this feels quite unsafe, add error handling..
    }
}


// implement SR03 settings (application sheet)
// implement switching between continuous and single shot measuring
// code


pub enum ToFAddresses {
    Address29 = 0x29,
}

pub enum ToFError {
    I2cError(I2cError)
}

// private registers array (if you have a better idea please i beg you)
const PRIVATE_REGISTERS: [u8; 30] = [
    0x0207,
    0x0208,
    0x0096,
    0x0097,
    0x00e3,
    0x00e4,
    0x00e5,
    0x00e6,
    0x00e7,
    0x00f5,
    0x00d9,
    0x00db,
    0x00dc,
    0x00dd,
    0x009f,
    0x00a3,
    0x00b7,
    0x00bb,
    0x00b2,
    0x00ca,
    0x0198,
    0x01b0,
    0x01ad,
    0x00ff,
    0x0100,
    0x0199,
    0x01a6,
    0x01ac,
    0x01a7,
    0x0030
];
// create init values for private registers array
const PRIVATE_REGISTER_DATA: [u8; 30] = [
    0x01,
    0x01,
    0x00,
    0xfd,
    0x01,
    0x03,
    0x02,
    0x01,
    0x03,
    0x02,
    0x05,
    0xce,
    0x03,
    0xf8,
    0x00,
    0x3c,
    0x00,
    0x3c,
    0x09,
    0x09,
    0x01,
    0x17,
    0x00,
    0x05,
    0x05,
    0x05,
    0x1b,
    0x3e,
    0x1f,
    0x00
];



