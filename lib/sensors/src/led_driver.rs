use hyped_io::i2c::{HypedI2c, I2cError};

/// LedDriver implements logic and required registers and addresses from the LP503x LED driver

/// LED driver is set to colour bank control mode, LED colour and brightness can only be set
/// for those in bank control mode.
/// All LEDs in bank control mode share the same colour and brightness changes unless set
/// off bank control mode

/// When setting colours and brightness of LEDs, use the set_led_colours() function.
///     LEDs needing to be set are controlled by the addresses in either LED_CONFIG0 or LED_CONFIG1 registers
///     These addresses can be toggled with hexadecimal (E.G: registers 11101000 are set by the hex value E8)

/// link to data sheet (https://www.ti.com/lit/ds/symlink/lp5036.pdf?HQS=dis-mous-null-mousermode-dsf-pf-null-wwe&ts=1698441544495&ref_url=https%253A%252F%252Fwww.mouser.co.uk%252F)

pub struct LedDriver<'a, T: HypedI2c + 'a> {
    i2c: &'a mut T,
    device_address: u8,
}

impl<'a, T: HypedI2c> LedDriver<'a, T> {
    /// Create new instance of LED driver and attempt to configure
    pub fn new(i2c: &'a mut T, device_address: LedDriverAddresses) -> Result<Self, LedDriverError> {
        let device_address = device_address as u8;

        // set up led driver enabling Chip_En in DEVICE_CONFIG1
        match i2c.write_byte_to_register(device_address, DEVICE_CONFIG0, CHIP_EN) {
            Ok(_) => Ok(Self {
                i2c,
                device_address,
            }),
            Err(e) => Err(LedDriverError::I2cError(e)),
        }
    }

    /// Device reset LED driver
    pub fn reset(&mut self) -> Result<(), LedDriverError> {
        match self
            .i2c
            .write_byte_to_register(self.device_address, RESET, 0xFF)
        {
            Ok(_) => (),
            Err(e) => return Err(LedDriverError::I2cError(e)),
        }

        Ok(())
    }

    /// set bank colour and brightness
    pub fn set_led_colour(
        &mut self,
        led_configx: u8,
        ledx_bank_en: u8,
        a_colour: u8,
        b_colour: u8,
        c_colour: u8,
        brightness: u8,
    ) -> Result<(), LedDriverError> {
        // enable LEDs, x, to bank control mode - can be multiple (see documentation at top)
        match self
            .i2c
            .write_byte_to_register(self.device_address, led_configx, ledx_bank_en)
        {
            Ok(_) => (),
            Err(e) => return Err(LedDriverError::I2cError(e)),
        }

        // change bank A colour value
        match self
            .i2c
            .write_byte_to_register(self.device_address, BANK_A_COLOUR, a_colour)
        {
            Ok(_) => (),
            Err(e) => return Err(LedDriverError::I2cError(e)),
        }

        // change bank B colour value
        match self
            .i2c
            .write_byte_to_register(self.device_address, BANK_B_COLOUR, b_colour)
        {
            Ok(_) => (),
            Err(e) => return Err(LedDriverError::I2cError(e)),
        }

        // change bank C colour value
        match self
            .i2c
            .write_byte_to_register(self.device_address, BANK_C_COLOUR, c_colour)
        {
            Ok(_) => (),
            Err(e) => return Err(LedDriverError::I2cError(e)),
        }

        // set bank brightness value
        match self
            .i2c
            .write_byte_to_register(self.device_address, BANK_BRIGHTNESS, brightness)
        {
            Ok(_) => (),
            Err(e) => return Err(LedDriverError::I2cError(e)),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum LedDriverError {
    I2cError(I2cError),
}

pub enum LedDriverAddresses {
    // independent addresses
    Address30 = 0x30, // 011 0000 - ADDR1 and ADDR0 both GND
    Address31 = 0x31, // 011 0001
    Address32 = 0x32, // 011 0010
    Address33 = 0x33, // 011 0011

    // broadcast address
    AddressBroadcast = 0x1c, // 001 1100
}

// device registers
const DEVICE_CONFIG0: u8 = 0x00;

const RESET: u8 = 0x38;

// LED config registers
pub enum LedDriverConfigAddresses {
    LedConfig0 = 0x02,
    LedConfig1 = 0x03,


}

// 6th bit for DEVICE_CONFIG0, enables LP503x
const CHIP_EN: u8 = 0x20;

// colour bank registers
const BANK_BRIGHTNESS: u8 = 0x04;
const BANK_A_COLOUR: u8 = 0x05;
const BANK_B_COLOUR: u8 = 0x06;
const BANK_C_COLOUR: u8 = 0x07;

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::FnvIndexMap;
    use hyped_io::i2c::mock_i2c::MockI2c;

    #[test]
    fn test_config() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let _ = LedDriver::new(&mut i2c, LedDriverAddresses::Address30);

        // Verify values are written to required registers
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, DEVICE_CONFIG0)),
            Some(&Some(CHIP_EN))
        );
    }

    #[test]
    fn test_all_led_config0() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let mut led_driver = LedDriver::new(&mut i2c, LedDriverAddresses::Address30)
            .expect("could not create led_driver");
        let _ = led_driver.set_led_colour(
            LedDriverConfigAddresses::LedConfig0 as u8,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        );

        // Verify values are written to required registers
        assert_eq!(
            i2c.get_writes().get(&(
                LedDriverAddresses::Address30 as u8,
                LedDriverConfigAddresses::LedConfig0 as u8
            )),
            Some(&Some(0xFF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_A_COLOUR)),
            Some(&Some(0xFF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_B_COLOUR)),
            Some(&Some(0xFF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_C_COLOUR)),
            Some(&Some(0xFF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_BRIGHTNESS)),
            Some(&Some(0xFF))
        );
    }

    #[test]
    fn test_all_led_config1() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let mut led_driver = LedDriver::new(&mut i2c, LedDriverAddresses::Address30)
            .expect("could not create led_driver");
        let _ = led_driver.set_led_colour(
            LedDriverConfigAddresses::LedConfig1 as u8,
            0xF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        );

        // Verify values are written to required registers
        assert_eq!(
            i2c.get_writes().get(&(
                LedDriverAddresses::Address30 as u8,
                LedDriverConfigAddresses::LedConfig1 as u8
            )),
            Some(&Some(0xF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_A_COLOUR)),
            Some(&Some(0xFF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_B_COLOUR)),
            Some(&Some(0xFF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_C_COLOUR)),
            Some(&Some(0xFF))
        );
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, BANK_BRIGHTNESS)),
            Some(&Some(0xFF))
        );
    }

    #[test]
    fn test_reset() {
        let i2c_values = FnvIndexMap::new();
        let mut i2c = MockI2c::new(i2c_values);
        let mut led_driver = LedDriver::new(&mut i2c, LedDriverAddresses::Address30)
            .expect("could not create led_driver");
        let _pre = led_driver.set_led_colour(
            LedDriverConfigAddresses::LedConfig0 as u8,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            0xFF,
        );
        let _ = led_driver.reset();

        // Verify values in LedConfig0 are reset to default values
        assert_eq!(
            i2c.get_writes()
                .get(&(LedDriverAddresses::Address30 as u8, RESET)),
            Some(&Some(0xFF))
        );
    }
}
