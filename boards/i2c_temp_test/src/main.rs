#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// Registers
const STTS22H_CTRL: u8 = 0x04;
const STTS22H_DATA_TEMP_L: u8 = 0x06;
const STTS22H_DATA_TEMP_H: u8 = 0x07;
const STTS22H_STATUS: u8 = 0x05;
// Values to check the status of the temperature sensor from the STTS22H_STATUS register
const STTS22H_STATUS_BUSY: u8 = 0x01;
const STTS22H_TEMP_OVER_UPPER_LIMIT: u8 = 0x02;
const STTS22H_TEMP_UNDER_LOWER_LIMIT: u8 = 0x04;
// Sets the sensor to continuous mode, sets IF_ADD_INC, and sets the sampling rate to 200Hz
const STTS22H_CONFIG_SETTINGS: u8 = 0x3c;
const STTS22H_TEMP_SCALING_FACTOR: f64 = 0.01;

enum TemperatureAddresses {
    Address7f = 0x7f,
    Address38 = 0x38,
    Address3c = 0x3c,
    Address3e = 0x3e,
}

impl Into<u8> for TemperatureAddresses {
    fn into(self) -> u8 {
        match self {
            TemperatureAddresses::Address7f => 0x7f,
            TemperatureAddresses::Address38 => 0x38,
            TemperatureAddresses::Address3c => 0x3c,
            TemperatureAddresses::Address3e => 0x3e,
        }
    }
}

#[embassy_executor::task]
async fn temp() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut i2c = I2c::new_blocking(p.I2C2, p.PB10, p.PB11, Hertz(100_000), Default::default());

    let address: u8 = TemperatureAddresses::Address7f.into();

    // Set up the temperature sensor by sending the configuration settings to the STTS22H_CTRL register
    let write_result = i2c
        .blocking_write(address, [STTS22H_CTRL, STTS22H_CONFIG_SETTINGS].as_ref())
        .expect("Failed to configure the temperature sensor.");

    loop {
        // Read a temperature

        // Write the address of the STTS22H_DATA_TEMP_H register to the sensor to start reading the temperature
        let write_result = i2c
            .blocking_write(address, [STTS22H_DATA_TEMP_H].as_ref())
            .expect("Failed to write the address of the STTS22H_DATA_TEMP_H register.");

        let mut read = [];
        let temperature_high_byte = match i2c.blocking_read(address, &mut read) {
            Ok(_) => read[0],
            Err(_) => {
                defmt::info!("Failed to read the temperature high byte.");
                0
            }
        };

        // Write the address of the STTS22H_DATA_TEMP_L register to the sensor to continue reading the temperature
        let write_result = i2c
            .blocking_write(address, [STTS22H_DATA_TEMP_L].as_ref())
            .expect("Failed to write the address of the STTS22H_DATA_TEMP_L register.");

        let temperature_low_byte = match i2c.blocking_read(address, &mut read) {
            Ok(_) => read[0],
            Err(_) => {
                defmt::info!("Failed to read the temperature low byte.");
                0
            }
        };

        // Combine the high and low bytes to get the temperature
        let combined = (temperature_high_byte as u16) << 8 | temperature_low_byte as u16;
        let temperature = f64::from(combined) * STTS22H_TEMP_SCALING_FACTOR;

        defmt::info!("Temperature: {:?}", temperature);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    spawner.spawn(temp()).unwrap();

    // Some other tasks...

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
