use hyped_io::i2c::{HypedI2c, I2cError};

///ToF implements the logic to read Time of Flight data from the VL6180V1 Time of Flight
///sensor using I2C peripheral provided by the Hyped I2c trait.
///
/// The majority of this implementation was done by implementing code examples from the Application Sheet (see below)
/// into Rust code; this implementation should allow us to start single-shot and continuous measurements and read their results.
/// 
/// Data Sheet: https://www.st.com/en/imaging-and-photonics-solutions/vl6180.html#overview
/// 
/// Application Sheet: https://www.st.com/resource/en/application_note/an4545-vl6180x-basic-ranging-application-note-stmicroelectronics.pdf

pub struct TimeOfFlight<T: HypedI2c> {
    i2c: T,
    device_address: u8,
}

impl<T: HypedI2c> TimeOfFlight<T> {
    // Create a new instance of time of flight sensor, configure
    pub fn new(mut i2c: T, device_address: ToFAddresses) -> Result<Self, ToFError> { // SR03 Settings as seen in Application Sheet
        let device_address = device_address as u8;
        for i in 0..30 { // writing to private registers
        match i2c.write_byte_to_register_16(device_address, PRIVATE_REGISTERS[i], PRIVATE_REGISTER_DATA[i]) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (), // unused
        }
    } 
        // Recommended Public Registers now (see Application Sheet)
        match i2c.write_byte_to_register(device_address, SYS_MODE_GPIO1, SYS_MODE_GPIO_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        match i2c.write_byte_to_register_16(device_address, AVG_SAMPLE_PERIOD, AVG_SAMPLE_PERIOD_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        match i2c.write_byte_to_register(device_address, SYSRANGE_VHV_REPEAT_RATE, SYSRANGE_VHV_REPEAT_RATE_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        match i2c.write_byte_to_register(device_address, SYSRANGE_VHV_RECALIBRATE, SYSRANGE_VHV_RECALIBRATE_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        match i2c.write_byte_to_register(device_address, SYSRANGE_INTERMEASURE_PERIOD, SYSRANGE_INTERMEASURE_PERIOD_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        match i2c.write_byte_to_register(device_address, SYS_INTERRUPT_CONFIG_GPIO, SYS_INTERRUPT_CONFIG_GPIO_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        Ok(Self {
            i2c,
            device_address,
        })
    }

    pub fn start_ss_measure(mut i2c: T, device_address: ToFAddresses) -> Result<Self, ToFError> { // start single shot measurement
        let device_address = device_address as u8;
        match i2c.write_byte_to_register(device_address, SYSRANGE_START, SYSRANGE_START_SS_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        
        Ok (Self {
            i2c,
            device_address
        })
    }

    pub fn poll_range(&mut self) {
        let status_byte = 
        match self.i2c.read_byte(self.device_address, RESULT_INTERR_STATUS_GPIO) {
            Some(byte) => byte,
            None => 0, // not sure about returning 0 for None - will this somehow break stuff?
        };
        let mut range_status = status_byte & 0x07;
        while range_status != 0x04 {
            range_status = match self.i2c.read_byte(self.device_address, RESULT_INTERR_STATUS_GPIO) {
                Some(byte) => byte,
                None => 0,
            } & 0x07;
        }

    } // consider using a 10-iteration loop, each time it waits, say, 1-2 seconds. if the result is not ready after 10-20 seconds, return an error?

    pub fn read_range(&mut self) -> Option<u8> {
        let range_byte =
            match self.i2c.read_byte(self.device_address, RESULT_RANGE_VAL) {
                Some(byte) => byte,
                None => {
                    return None;
                }
            };
        Some(range_byte)
    }

    pub fn start_cts_measure(mut i2c: T, device_address: ToFAddresses) -> Result<Self, ToFError> { // start continuous measurement
        let device_address = device_address as u8;
        match i2c.write_byte_to_register(device_address, SYSRANGE_START, SYSRANGE_START_CTS_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        
        Ok (Self {
            i2c,
            device_address
        })
    }

    pub fn check_reset(&mut self) -> bool {
        let reset_value =  match self.i2c.read_byte(self.device_address, SYS_FRESH_OUT_RESET) {
            Some(byte) => byte,
            None => 0, // hopefully returning 0 is okay and won't break stuff
        };
        reset_value == 1
    }

    pub fn clear_interrupts(mut i2c: T, device_address: ToFAddresses) -> Result<Self, ToFError> { // at the end clear interrupts
        let device_address = device_address as u8;
        match i2c.write_byte_to_register(device_address, SYS_INTERRUPT_CLEAR, CLEAR_INTERRUPTS_VAL) {
            Err(e) =>  return Err(ToFError::I2cError(e)),
            _ => (),
        }
        
        Ok (Self {
            i2c,
            device_address
        })
    }
}



pub enum ToFAddresses {
    Address29 = 0x29,
}

pub enum ToFError {
    I2cError(I2cError)
}

// public register addresses
const SYS_MODE_GPIO1: u8 = 0x0011;
const AVG_SAMPLE_PERIOD: u16 = 0x010a;
// can't find reference to 0x003f address for light and dark gain in datasheet from the application note
const SYSRANGE_VHV_REPEAT_RATE: u8 = 0x031;
// can't find reference to 0x0041, see above.
const SYSRANGE_VHV_RECALIBRATE: u8 = 0x002e;
const SYSRANGE_INTERMEASURE_PERIOD: u8 = 0x01b;
// same story with 0x003e
const SYS_INTERRUPT_CONFIG_GPIO: u8 = 0x014;
const SYSRANGE_START: u8 = 0x018;
const RESULT_INTERR_STATUS_GPIO: u8 = 0x04f;
const SYS_FRESH_OUT_RESET: u8 = 0x016;
const SYS_INTERRUPT_CLEAR: u8 = 0x015;
// this one has VAL because that's what its' called in the docs, not actually a VALUE.
const RESULT_RANGE_VAL: u8 = 0x062;

// init values for public registers
const SYS_MODE_GPIO_VAL: u8 = 0x01;
const AVG_SAMPLE_PERIOD_VAL: u8 = 0x30;
const SYSRANGE_VHV_REPEAT_RATE_VAL: u8 = 0xFF;
const SYSRANGE_VHV_RECALIBRATE_VAL: u8 = 0x01;
const SYSRANGE_INTERMEASURE_PERIOD_VAL: u8 = 0x09;
const SYS_INTERRUPT_CONFIG_GPIO_VAL: u8 = 0x24;
const SYSRANGE_START_SS_VAL: u8 = 0x01;
const SYSRANGE_START_CTS_VAL: u8 = 0x03;
const CLEAR_INTERRUPTS_VAL: u8 = 0x07;

// private registers array
const PRIVATE_REGISTERS: [u16; 30] = [
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
// init values for private registers array
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
