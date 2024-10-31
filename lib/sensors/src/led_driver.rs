/// link to data sheet (https://www.ti.com/lit/ds/symlink/lp5036.pdf?HQS=dis-mous-null-mousermode-dsf-pf-null-wwe&ts=1698441544495&ref_url=https%253A%252F%252Fwww.mouser.co.uk%252F)

use hyped_io::i2c::{HypedI2c, I2cError};

pub struct LedDriver<T: HypedI2c> {
    i2c: T,
    device_address: u8,
}

impl<T: HypedI2c> LedDriver<T> {
    /// Create new instance of LED driver and attempt to configure
    pub fn new(mut i2c: T, device_address: LedDriverAddresses) -> Result<Self, LedDriverError>{
        let device_address = device_address as u8;

        // set up led driver by sending config settings to [tbd control register]
        match i2c.write_byte_to_register(device_address, DEVICE_CONFIG0, DEVICE_CONFIG1){   // incorrect CHANGE LATER
            Ok(_) => Ok(Self {
                i2c,
                device_address,
            }),
            Err(e) => Err(LedDriverError::I2cError(e)),
        }
    }

    /// set LED colour
    pub fn set_led_colour(mut i2c: T, device_address: u8, register_address: u8, bank_en: u8, a_colour: u8, b_colour: u8, c_colour:u8){

        // toggle LEDx_Bank_EN to bank control mode (bank_en)
        match i2c.write_byte_to_register(device_address, register_address, bank_en){
            Ok(_) => Ok(Self {
                i2c,
                device_address,
            }),
            Err(e) => Err(LedDriverError::I2cError(e)),
        };

        // change bank A colour value
        match i2c.write_byte_to_register(device_address, BANK_A_COLOUR, a_colour){
            Ok(_) => Ok(Self {
                i2c,
                device_address,
            }),
            Err(e) => Err(LedDriverError::I2cError(e)),
        };

        // change bank B colour value
        match i2c.write_byte_to_register(device_address, BANK_B_COLOUR, b_colour){
            Ok(_) => Ok(Self {
                i2c,
                device_address,
            }),
            Err(e) => Err(LedDriverError::I2cError(e)),
        };

        // change bank C colour value
        match i2c.write_byte_to_register(device_address, BANK_C_COLOUR, c_colour){
            Ok(_) => Ok(Self {
                i2c,
                device_address,
            }),
            Err(e) => Err(LedDriverError::I2cError(e)),
        };

    }
    // TODO: receiving signal function
}


pub enum LedDriverError{
    I2cError(I2cError),
}

pub enum LedDriverAddresses{
    // tbd
}

// device registers
const DEVICE_CONFIG0: u8 = 0x00;
const DEVICE_CONFIG1: u8 = 0x01;

// LED config registers
const LED_CONFIG0: u8 = 0x02;
const LED_CONFIG1: u8 = 0x03;


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