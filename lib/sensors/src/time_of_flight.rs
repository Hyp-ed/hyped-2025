use defmt;
use hyped_i2c::{HypedI2c, I2cError};
/// time_of_flight implements the logic to read Time of Flight data from the VL6180V1 Time of Flight
/// sensor using I2C peripheral provided by the Hyped I2c trait.
///
/// The majority of this implementation was done by implementing code examples from the Application Sheet (see below)
/// into Rust code; this implementation allows us to start single-shot measurements and read their results. The sensor's
/// task in /tasks/time_of_flight.rs is a test task that reads range via single-shot measurement.
///
/// Data Sheet: https://www.st.com/en/imaging-and-photonics-solutions/vl6180.html#overview
///
/// Application Sheet: https://www.st.com/resource/en/application_note/an4545-vl6180x-basic-ranging-application-note-stmicroelectronics.pdf
///
/// There is a lot of 'If Let' uses in this code, refer to the Rust docs for more details: https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html

pub struct TimeOfFlight<'a, T: HypedI2c> {
    i2c: &'a mut T,
    device_address: u8,
}

impl<'a, T: HypedI2c> TimeOfFlight<'a, T> {
    /// Create a new instance of time of flight sensor, configure. This function
    /// will create a new instance of the time of flight sensor and check whether it is
    /// 'fresh out of reset' or not. If needed, it'll load the config onto the board and if the
    /// config isn't there, it'll skip loading and return the new sensor instance.
    pub fn new(
        i2c: &'a mut T,
        device_address: TimeOfFlightAddresses,
    ) -> Result<Self, TimeOfFlightError> {
        let device_address = device_address as u8;
        // Check that the sensor has powered up
        // Note: the sensor will either need to be power cycled or the SYS_FRESH_OUT_RESET register will need to be written to 0x01 if it has already been configured
        let boot_status = i2c
            .read_byte_16(device_address, SYS_FRESH_OUT_RESET)
            .unwrap_or_default();
        let mut time_of_flight = Self {
            i2c,
            device_address,
        };
        // Check boot status; if it's not freshly reset, boot is complete. If it is, load all configs.
        if boot_status == 0 {
            defmt::info!("Time Of Flight sensor booted");
        } else {
            time_of_flight.load_config(device_address)?;
        }

        Ok(time_of_flight)
    }

    /// Private method used to load all of the configs according to SR03 settings
    fn load_config(&mut self, device_address: u8) -> Result<(), TimeOfFlightError> {
        let i2c = &mut self.i2c;
        // SR03 Settings as seen in Application Sheet on page 24
        // Write to private registers
        for (reg, val) in PRIVATE_REGISTERS {
            if let Err(e) = i2c.write_byte_to_register_16(device_address, reg, val) {
                return Err(TimeOfFlightError::I2cError(e));
            }
        }

        // Recommended Public Registers (see Application Sheet)
        if let Err(e) =
            i2c.write_byte_to_register_16(device_address, SYS_MODE_GPIO1, SYS_MODE_GPIO_VAL)
        {
            return Err(TimeOfFlightError::I2cError(e));
        }

        if let Err(e) =
            i2c.write_byte_to_register_16(device_address, AVG_SAMPLE_PERIOD, AVG_SAMPLE_PERIOD_VAL)
        {
            return Err(TimeOfFlightError::I2cError(e));
        }

        if let Err(e) = i2c.write_byte_to_register_16(
            device_address,
            SYSRANGE_VHV_REPEAT_RATE,
            SYSRANGE_VHV_REPEAT_RATE_VAL,
        ) {
            return Err(TimeOfFlightError::I2cError(e));
        }

        if let Err(e) = i2c.write_byte_to_register_16(
            device_address,
            SYSRANGE_VHV_RECALIBRATE,
            SYSRANGE_VHV_RECALIBRATE_VAL,
        ) {
            return Err(TimeOfFlightError::I2cError(e));
        }

        if let Err(e) = i2c.write_byte_to_register_16(
            device_address,
            SYSRANGE_INTERMEASURE_PERIOD,
            SYSRANGE_INTERMEASURE_PERIOD_VAL,
        ) {
            return Err(TimeOfFlightError::I2cError(e));
        }

        if let Err(e) = i2c.write_byte_to_register_16(
            device_address,
            SYS_INTERRUPT_CONFIG_GPIO,
            SYS_INTERRUPT_CONFIG_GPIO_VAL,
        ) {
            return Err(TimeOfFlightError::I2cError(e));
        }
        defmt::info!("Time Of Flight sensor configured");

        // Write 0x00 to SYS_FRESH_OUT_RESET to indicate that the sensor has been configured
        // Note: as above, this means that the sensor will need to be power cycled or this register will need to be written to 0x01 to reconfigure the sensor
        if let Err(e) = i2c.write_byte_to_register_16(device_address, SYS_FRESH_OUT_RESET, 0x00) {
            return Err(TimeOfFlightError::I2cError(e));
        };
        Ok(())
    }

    /// The VL6180X Time Of Flight Sensor has 2 modes of measurement, 'single shot' and 'continuous'
    /// Single shot will take a single measurement of the range and return it, re-entering standby mode, whilst continuous
    /// will take a measurement at a user-defined period and keep doing this until the user tells it to stop (at which point it returns to standby).
    /// Continuous mode is apparently more suited for averaging range results, so we'll be using single shot at the moment.
    /// You can find details about these modes on page 6 of the Application Note:
    /// https://www.st.com/resource/en/application_note/an4545-vl6180x-basic-ranging-application-note-stmicroelectronics.pdf

    pub fn single_shot_measurement(&mut self) -> Result<u8, TimeOfFlightError> {
        if let Err(e) = self.i2c.write_byte_to_register_16(
            self.device_address,
            SYSRANGE_START,
            SYSRANGE_START_SS_VAL,
        ) {
            return Err(TimeOfFlightError::I2cError(e));
        }
        let status_byte = self
            .i2c
            .read_byte_16(self.device_address, RESULT_INTERR_STATUS_GPIO)
            .unwrap_or(0);
        let mut range_status = status_byte & 0x07;
        while range_status != 0x04 {
            range_status = self
                .i2c
                .read_byte_16(self.device_address, RESULT_INTERR_STATUS_GPIO)
                .unwrap_or_default()
                & 0x07;
        }
        // Read range from the RESULT_RANGE_VAL register and return it
        let range = self
            .i2c
            .read_byte_16(self.device_address, RESULT_RANGE_VAL)
            .ok_or(TimeOfFlightError::I2cError(I2cError::Unknown));
        // For good practice, interrupts have to be cleared at the end of the program 'loop' each time. See Application Sheet page 22
        if let Err(e) = self.i2c.write_byte_to_register_16(
            self.device_address,
            SYS_INTERRUPT_CLEAR,
            CLEAR_INTERRUPTS_VAL,
        ) {
            return Err(TimeOfFlightError::I2cError(e));
        }
        return range;
    }
}

pub enum TimeOfFlightAddresses {
    Address29 = 0x29,
}

#[derive(Debug)]
pub enum TimeOfFlightError {
    I2cError(I2cError),
}

// Public register addresses
const SYS_MODE_GPIO1: u16 = 0x0011;
const AVG_SAMPLE_PERIOD: u16 = 0x010a;
const SYSRANGE_VHV_REPEAT_RATE: u16 = 0x031;
const SYSRANGE_VHV_RECALIBRATE: u16 = 0x002e;
const SYSRANGE_INTERMEASURE_PERIOD: u16 = 0x01b;
const SYS_INTERRUPT_CONFIG_GPIO: u16 = 0x014;
const SYSRANGE_START: u16 = 0x018;
const RESULT_INTERR_STATUS_GPIO: u16 = 0x04f;
const SYS_FRESH_OUT_RESET: u16 = 0x016;
const SYS_INTERRUPT_CLEAR: u16 = 0x015;
// RESULT_RANGE_VAL is actually a register; that's what the docs call it
const RESULT_RANGE_VAL: u16 = 0x062;

// Values for public registers
const SYS_MODE_GPIO_VAL: u8 = 0x10;
const AVG_SAMPLE_PERIOD_VAL: u8 = 0x30;
const SYSRANGE_VHV_REPEAT_RATE_VAL: u8 = 0xFF;
const SYSRANGE_VHV_RECALIBRATE_VAL: u8 = 0x01;
const SYSRANGE_INTERMEASURE_PERIOD_VAL: u8 = 0x09;
const SYS_INTERRUPT_CONFIG_GPIO_VAL: u8 = 0x24;
const SYSRANGE_START_SS_VAL: u8 = 0x01;
const CLEAR_INTERRUPTS_VAL: u8 = 0x07;

// Private registers (16 bit address, 8 bit value)
const PRIVATE_REGISTERS: [(u16, u8); 30] = [
    (0x0207, 0x01),
    (0x0208, 0x01),
    (0x0096, 0x00),
    (0x0097, 0xfd),
    (0x00e3, 0x01),
    (0x00e4, 0x03),
    (0x00e5, 0x02),
    (0x00e6, 0x01),
    (0x00e7, 0x03),
    (0x00f5, 0x02),
    (0x00d9, 0x05),
    (0x00db, 0xce),
    (0x00dc, 0x03),
    (0x00dd, 0xf8),
    (0x009f, 0x00),
    (0x00a3, 0x3c),
    (0x00b7, 0x00),
    (0x00bb, 0x3c),
    (0x00b2, 0x09),
    (0x00ca, 0x09),
    (0x0198, 0x01),
    (0x01b0, 0x17),
    (0x01ad, 0x00),
    (0x00ff, 0x05),
    (0x0100, 0x05),
    (0x0199, 0x05),
    (0x01a6, 0x1b),
    (0x01ac, 0x3e),
    (0x01a7, 0x1f),
    (0x0030, 0x00),
];

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    use super::*;
    use embassy_sync::blocking_mutex::Mutex;
    use heapless::FnvIndexMap;
    use hyped_i2c::mock_i2c::MockI2c;

    #[test]
    fn test_time_of_flight_config() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (TimeOfFlightAddresses::Address29 as u8, SYS_FRESH_OUT_RESET),
            Some(1),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let _ = TimeOfFlight::new(&mut i2c, TimeOfFlightAddresses::Address29);

        for (reg, val) in PRIVATE_REGISTERS {
            assert_eq!(
                i2c.get_writes()
                    .get(&(TimeOfFlightAddresses::Address29 as u8, reg)),
                Some(&Some(val))
            )
        }
        assert_eq!(
            i2c.get_writes()
                .get(&(TimeOfFlightAddresses::Address29 as u8, SYS_MODE_GPIO1)),
            Some(&Some(SYS_MODE_GPIO_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                TimeOfFlightAddresses::Address29 as u8,
                SYSRANGE_VHV_REPEAT_RATE
            )),
            Some(&Some(SYSRANGE_VHV_REPEAT_RATE_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                TimeOfFlightAddresses::Address29 as u8,
                SYSRANGE_VHV_RECALIBRATE
            )),
            Some(&Some(SYSRANGE_VHV_RECALIBRATE_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                TimeOfFlightAddresses::Address29 as u8,
                SYSRANGE_INTERMEASURE_PERIOD
            )),
            Some(&Some(SYSRANGE_INTERMEASURE_PERIOD_VAL))
        );
        assert_eq!(
            i2c.get_writes().get(&(
                TimeOfFlightAddresses::Address29 as u8,
                SYS_INTERRUPT_CONFIG_GPIO
            )),
            Some(&Some(SYS_INTERRUPT_CONFIG_GPIO_VAL))
        );
    }

    #[test]
    fn test_single_shot() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (TimeOfFlightAddresses::Address29 as u8, SYS_FRESH_OUT_RESET),
            Some(1),
        );
        let _ = i2c_values.insert(
            (
                TimeOfFlightAddresses::Address29 as u8,
                RESULT_INTERR_STATUS_GPIO,
            ),
            Some(0x04),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut time_of_flight =
            TimeOfFlight::new(&mut i2c, TimeOfFlightAddresses::Address29).unwrap();
        time_of_flight.single_shot_measurement().unwrap();
        assert_eq!(
            i2c.get_writes()
                .get(&(TimeOfFlightAddresses::Address29 as u8, SYSRANGE_START)),
            Some(&Some(SYSRANGE_START_SS_VAL))
        );
    }

    #[test]
    fn test_clear_interr() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (TimeOfFlightAddresses::Address29 as u8, SYS_FRESH_OUT_RESET),
            Some(1),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut time_of_flight =
            TimeOfFlight::new(&mut i2c, TimeOfFlightAddresses::Address29).unwrap();
        time_of_flight.clear_interrupts().unwrap();
        assert_eq!(
            i2c.get_writes()
                .get(&(TimeOfFlightAddresses::Address29 as u8, SYS_INTERRUPT_CLEAR)),
            Some(&Some(CLEAR_INTERRUPTS_VAL))
        );
    }

    #[test]
    fn test_range_read_255() {
        let mut i2c_values = FnvIndexMap::new();
        let _ = i2c_values.insert(
            (TimeOfFlightAddresses::Address29 as u8, SYS_FRESH_OUT_RESET),
            Some(1),
        );
        let _ = i2c_values.insert(
            (TimeOfFlightAddresses::Address29 as u8, RESULT_RANGE_VAL),
            Some(255),
        );
        let i2c_values = Mutex::new(RefCell::new(i2c_values));
        let mut i2c = MockI2c::new(&i2c_values);
        let mut time_of_flight =
            TimeOfFlight::new(&mut i2c, TimeOfFlightAddresses::Address29).unwrap();
        // assert_eq!(time_of_flight.read_range(), Some(255));
    }
}
