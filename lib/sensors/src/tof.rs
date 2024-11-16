use hyped_io::i2c::{HypedI2c, I2cError};

///ToF implements the logic to read Time of Flight data from the VL6180V1 Time of Flight
///sensor using I2C peripheral provided by the Hyped I2c trait.
///
/// The majority of this implementation was done by implementing code examples from the Application Sheet (see below)
/// into Rust code; this implementation should allow us to start single-shot and continuous measurements and read their results.
/// Switching between single-shot and continuous measurements is outlined at the end of page 22 of the Application Sheet. The sensor's
/// task in /tasks/tof.rs is a test task that reads range via single-shot measurement.
///
/// Data Sheet: https://www.st.com/en/imaging-and-photonics-solutions/vl6180.html#overview
///
/// Application Sheet: https://www.st.com/resource/en/application_note/an4545-vl6180x-basic-ranging-application-note-stmicroelectronics.pdf

pub struct TimeOfFlight<'a, T: HypedI2c + 'a> {
    i2c: &'a mut T,
    device_address: u8,
}

impl<'a, T: HypedI2c> TimeOfFlight<'a, T> {
    // Create a new instance of time of flight sensor, configure
    pub fn new(i2c: &'a mut T, device_address: ToFAddresses) -> Result<Self, ToFError> {
        // SR03 Settings as seen in Application Sheet
        let device_address = device_address as u8;
        for (reg, val) in PRIVATE_REGISTERS_u8 {
            // writing to private registers
            if let Err(e) = i2c.write_byte_to_register(
                device_address,
                reg,
                val,
            ) {
                panic!("Error writing private registers u8s REG: {:?}", reg);
            }
        }
        for (reg,val) in PRIVATE_REGISTERS_u16 {
            // writing to private registers u16
            if let Err(e) = i2c.write_byte_to_register_16(
                device_address,
                reg,
                val,
            ) {
                panic!("Error writing private registers u16s");
            }
        }
        // Recommended Public Registers now (see Application Sheet)
        if let Err(e) =
            i2c.write_byte_to_register(device_address, SYS_MODE_GPIO1, SYS_MODE_GPIO_VAL)
        {
            panic!("Error writing SYS_MODE_GPIO_1 register");
        }
        if let Err(e) =
            i2c.write_byte_to_register_16(device_address, AVG_SAMPLE_PERIOD, AVG_SAMPLE_PERIOD_VAL)
        {
            panic!("Error writing AVG_SAMPLE_PERIOD REGISTER");
        }
        if let Err(e) = i2c.write_byte_to_register(
            device_address,
            SYSRANGE_VHV_REPEAT_RATE,
            SYSRANGE_VHV_REPEAT_RATE_VAL,
        ) {
            panic!("Error writing SYSRANGE_VHV_REPEAT_RATE REGISTER");
        }
        if let Err(e) = i2c.write_byte_to_register(
            device_address,
            SYSRANGE_VHV_RECALIBRATE,
            SYSRANGE_VHV_RECALIBRATE_VAL,
        ) {
            panic!("Error writing SYSRANGE_VHV_RECALIBRATE REGISTER");
        }
        if let Err(e) = i2c.write_byte_to_register(
            device_address,
            SYSRANGE_INTERMEASURE_PERIOD,
            SYSRANGE_INTERMEASURE_PERIOD_VAL,
        ) {
            panic!("Error writing SYSRANGE_INTERMEASURE_PERIOD REGISTER");
        }
        if let Err(e) = i2c.write_byte_to_register(
            device_address,
            SYS_INTERRUPT_CONFIG_GPIO,
            SYS_INTERRUPT_CONFIG_GPIO_VAL,
        ) {
            panic!("Error writing SYS_INTERRUPT_CONFIG_GPIO REGISTER");
        }
        Ok(Self {
            i2c,
            device_address,
        })
    }

    pub fn start_ss_measure(&mut self) -> Result<i32, ToFError> {
        // start single shot measurement
        if let Err(e) = self.i2c.write_byte_to_register(
            self.device_address,
            SYSRANGE_START,
            SYSRANGE_START_SS_VAL,
        ) {
            return Err(ToFError::I2cError(e));
        }
        Ok(1)
    }

    pub fn poll_range(&mut self) {
        let status_byte = self
            .i2c
            .read_byte(self.device_address, RESULT_INTERR_STATUS_GPIO)
            .unwrap_or(0);
        let mut range_status = status_byte & 0x07;
        while range_status != 0x04 {
            range_status = self
                .i2c
                .read_byte(self.device_address, RESULT_INTERR_STATUS_GPIO)
                .unwrap_or_default()
                & 0x07;
        }
    }

    pub fn read_range(&mut self) -> Option<u8> {
        let range_byte = match self.i2c.read_byte(self.device_address, RESULT_RANGE_VAL) {
            Some(byte) => byte,
            None => {
                return None;
            }
        };
        Some(range_byte)
    }

    pub fn start_cts_measure(&mut self) -> Result<i32, ToFError> {
        // start continuous measurement
        if let Err(e) = self.i2c.write_byte_to_register(
            self.device_address,
            SYSRANGE_START,
            SYSRANGE_START_CTS_VAL,
        ) {
            return Err(ToFError::I2cError(e));
        }
        Ok(1)
    }

    pub fn check_reset(&mut self) -> bool {
        let reset_value = self
            .i2c
            .read_byte(self.device_address, SYS_FRESH_OUT_RESET)
            .unwrap_or(0);
        reset_value == 1
    }

    pub fn clear_interrupts(&mut self) -> Result<i32, ToFError> {
        // at the end clear interrupts
        if let Err(e) = self.i2c.write_byte_to_register(
            self.device_address,
            SYS_INTERRUPT_CLEAR,
            CLEAR_INTERRUPTS_VAL,
        ) {
            return Err(ToFError::I2cError(e));
        }
        Ok(1)
    }
}

pub enum ToFAddresses {
    Address29 = 0x29,
}

#[derive(Debug)]
pub enum ToFError {
    I2cError(I2cError),
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

// private registers tuples

const PRIVATE_REGISTERS_u8: [(u8,u8); 16] = [
    (0x0096,0x00),
    (0x0097,0xfd),
    (0x00e3,0x01),
    (0x00e4,0x03),
    (0x00e5,0x02),
    (0x00e6,0x01),
    (0x00e7,0x03),
    (0x00f5,0x02),
    (0x00d9,0x05),
    (0x00db,0xce),
    (0x00dc,0x03),
    (0x00dd,0xf8),
    (0x009f,0x00),
    (0x00a3,0x3c),
    (0x00b7,0x00),
    (0x00bb,0x3c),
];

const PRIVATE_REGISTERS_u16: [(u16,u8); 10] = [
    (0x0207,0x01),
    (0x0208,0x01),
    (0x0198,0x01),
    (0x01b0,0x17),
    (0x01ad,0x00),
    (0x0100,0x05),
    (0x0199,0x05),
    (0x01a6,0x1b),
    (0x01ac,0x3e),
    (0x01a7,0x1f)
];

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::FnvIndexMap;
    use hyped_io::i2c::mock_i2c::MockI2c;

    #[test]
    fn test_tof_config() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let _ = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29);
        for (reg,val) in PRIVATE_REGISTERS_u8 {
            assert_eq!(
                i2c.get_writes()
                    .get(&(ToFAddresses::Address29 as u8, reg.into())),
                Some(&Some(val))
            )
        }
        for (reg,val) in PRIVATE_REGISTERS_u16 {
            assert_eq!(
                i2c.get_writes()
                    .get(&(ToFAddresses::Address29 as u8, reg.into())),
                Some(&Some(val))
            )
        }
        assert_eq!(
            i2c.get_writes()
                .get(&(ToFAddresses::Address29 as u8, SYS_MODE_GPIO1.into())),
            Some(&Some(SYS_MODE_GPIO_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                ToFAddresses::Address29 as u8,
                SYSRANGE_VHV_REPEAT_RATE.into()
            )),
            Some(&Some(SYSRANGE_VHV_REPEAT_RATE_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                ToFAddresses::Address29 as u8,
                SYSRANGE_VHV_RECALIBRATE.into()
            )),
            Some(&Some(SYSRANGE_VHV_RECALIBRATE_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                ToFAddresses::Address29 as u8,
                SYSRANGE_INTERMEASURE_PERIOD.into()
            )),
            Some(&Some(SYSRANGE_INTERMEASURE_PERIOD_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                ToFAddresses::Address29 as u8,
                SYS_INTERRUPT_CONFIG_GPIO.into()
            )),
            Some(&Some(SYS_INTERRUPT_CONFIG_GPIO_VAL))
        );
    }

    #[test]
    fn test_start_ss() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let _ = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29);
        assert_eq!(
            i2c.get_writes()
                .get(&(ToFAddresses::Address29 as u8, SYSRANGE_START.into())),
            Some(&Some(SYSRANGE_START_SS_VAL))
        );
    }

    #[test]
    fn test_start_cts() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let _ = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29);
        assert_eq!(
            i2c.get_writes()
                .get(&(ToFAddresses::Address29 as u8, SYSRANGE_START.into())),
            Some(&Some(SYSRANGE_START_CTS_VAL))
        );
    }

    #[test]
    fn test_clear_interr() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let _ = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29);
        assert_eq!(
            i2c.get_writes()
                .get(&(ToFAddresses::Address29 as u8, SYS_INTERRUPT_CLEAR.into())),
            Some(&Some(CLEAR_INTERRUPTS_VAL))
        );
    }

    #[test]
    fn test_range_read_0() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (ToFAddresses::Address29 as u8, RESULT_RANGE_VAL as u16),
            Some(0),
        );
        let mut i2c = MockI2c::new(i2c_values);
        let mut tof = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29).unwrap();
        assert_eq!(tof.read_range(), Some(0));
    }

    #[test]
    fn test_range_read_200() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (ToFAddresses::Address29 as u8, RESULT_RANGE_VAL as u16),
            Some(200),
        );
        let mut i2c = MockI2c::new(i2c_values);
        let tof = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29);
        match tof {
            Ok(mut tof) => {
                assert_eq!(tof.read_range(), Some(200))
            }
            Err(e) => {
                panic!("Failed to create a Time of Flight object {:?}", e);
            }
        }
    }

    #[test]
    fn test_range_read_255() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (ToFAddresses::Address29 as u8, RESULT_RANGE_VAL as u16),
            Some(255),
        );
        let mut i2c = MockI2c::new(i2c_values);
        let mut tof = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29).unwrap();
        assert_eq!(tof.read_range(), Some(255));
    }

    #[test]
    fn test_read_reset() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (ToFAddresses::Address29 as u8, SYS_FRESH_OUT_RESET as u16),
            Some(1),
        );
        let mut i2c = MockI2c::new(i2c_values);
        let mut tof = TimeOfFlight::new(&mut i2c, ToFAddresses::Address29).unwrap();
        assert!(tof.check_reset());
    }
}
