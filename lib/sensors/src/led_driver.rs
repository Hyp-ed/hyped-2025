/// LED Driver

/// link to data sheet (https://www.ti.com/lit/ds/symlink/lp5036.pdf?HQS=dis-mous-null-mousermode-dsf-pf-null-wwe&ts=1698441544495&ref_url=https%253A%252F%252Fwww.mouser.co.uk%252F)

use hyped_io::i2c::{HypedI2c, I2cError};

pub struct LedDriver<'a, T: HypedI2c + 'a> {
    i2c: &'a mut T,
    device_address: u8,
}

impl<'a, T: HypedI2c> LedDriver<'a, T> {
    /// Create new instance of LED driver and attempt to configure
    pub fn new(
        i2c: &'a mut T,
        device_address: LedDriverAddresses)
        -> Result<Self, LedDriverError>{

        let device_address = device_address as u8;

        // set up led driver enabling Chip_En in DEVICE_CONFIG1
        match i2c.write_byte_to_register(device_address, DEVICE_CONFIG0, CHIP_EN){
            Ok(_) => Ok(Self {
                i2c,
                device_address,
            }),
            Err(e) => Err(LedDriverError::I2cError(e)),
        }
    }

    /// set LED to bank control mode
    pub fn set_led_to_bank(
        self: &mut Self,
        led_configx: u8,
        ledx_bank_en: u8,
        ) -> Result<(), LedDriverError>{
            // toggle LEDx_Bank_EN to bank control mode
            match self
            .i2c
            .write_byte_to_register(self.device_address, led_configx, ledx_bank_en)
            {
                Ok(_) => (),
                Err(e) => return Err(LedDriverError::I2cError(e)),
            }

            Ok(())
        }

    /// set bank colour and brightness
    pub fn set_bank_colour(
        self: &mut Self,
        a_colour: u8,
        b_colour: u8,
        c_colour: u8, brightness: u8
    ) -> Result<(), LedDriverError>{
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


pub enum LedDriverError{
    I2cError(I2cError),
}

pub enum LedDriverAddresses{
    // independent addresses
    Address30 = 0x30,
    Address31 = 0x31,
    Address32 = 0x32,
    Address33 = 0x33,

    // broadcast address
    AddressBroadcast = 0x1c
}

// device registers
const DEVICE_CONFIG0: u8 = 0x00;
// const DEVICE_CONFIG1: u8 = 0x01;    // may not be needed

// LED config registers
const LED_CONFIG0: u8 = 0x02;
const LED_CONFIG1: u8 = 0x03;


/// LED bank EN addresses (hexadecimal)
// LED_CONFIG0
const LED0_BANK_EN: u8 = 0x1;
const LED1_BANK_EN: u8 = 0x2;
const LED2_BANK_EN: u8 = 0x4;
const LED3_BANK_EN: u8 = 0x8;
const LED4_BANK_EN: u8 = 0x10;
const LED5_BANK_EN: u8 = 0x20;
const LED6_BANK_EN: u8 = 0x40;
const LED7_BANK_EN: u8 = 0x80;

// LED_CONFIG1
const LED8_BANK_EN: u8 = 0x1;
const LED9_BANK_EN: u8 = 0x2;

/* FOR REVIEWER: wasn't sure if i should have made the LEDx_BANK_ENs enumerated values.
    Since values are repeated (some being handled by LED_CONFIG0 and others LED_CONFIG1),
    would need to make 2 separate enums, which could be unnecessarily confusing for end users.
    Please could I ask for a second opinion on this?
*/

// 6th bit for DEVICE_CONFIG0, enables LP503x
const CHIP_EN: u8 = 0x20;

// colour bank registers
const BANK_BRIGHTNESS: u8 = 0x04;
const BANK_A_COLOUR: u8 = 0x05;
const BANK_B_COLOUR: u8 = 0x06;
const BANK_C_COLOUR: u8 = 0x07;







// "3 programmable banks for easy software control of each colour"
//  LED bank control provides easy programming approach to controlling LED lighting
    // instead of controlling each individual led separately , which takes heavy resource-power

// configure an led state (independent control, bank control) through LEDx_Bank_EN register
    // LEDx_Bank_EN = 0 (default), LED is controlled independently by related colour-mixing and intensity-control registers
    // LEDx_Bank_EN = 1, LP503x device drives LED in LED bank-control mode

    // LED bank has its own independent PWM control scheme, same structure as PWM scheme of each channel

// when channel in bank-control mode, the related colour mixing and intensity control is governed by
    // bank control registers (BANK_A_COLOR, BANK_B_COLOR, BANK_C_COLOR, BANK_BRIGHTNESS) regardless
    // of the inputs on its own color-mixing and intensity-control registers