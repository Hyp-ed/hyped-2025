use core::cell::RefCell;

use crate::io::i2c::Stm32f767ziI2c;
use defmt_rtt as _;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_sync::blocking_mutex::Mutex;
use hyped_io::i2c_mux::{I2cMux, DEFAULT_MUX_ADDRESS};
use hyped_sensors::temperature::{Temperature, TemperatureAddresses};

#[embassy_executor::task]
pub async fn example() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());

    let hyped_i2c = Mutex::new(RefCell::new(Stm32f767ziI2c::new(i2c)));

    // Let's say we have 4 temperature sensors connected to the multiplexer
    let mut i2c_mux_1 =
        I2cMux::new(&hyped_i2c, 0, DEFAULT_MUX_ADDRESS).expect("Failed to create I2C multiplexer.");
    let mut temp_1 = Temperature::new(&mut i2c_mux_1, TemperatureAddresses::Address3c)
        .expect("Failed to create temperature sensor.");

    let mut i2c_mux_2 =
        I2cMux::new(&hyped_i2c, 1, DEFAULT_MUX_ADDRESS).expect("Failed to create I2C multiplexer.");
    let mut temp_2 = Temperature::new(&mut i2c_mux_2, TemperatureAddresses::Address3c)
        .expect("Failed to create temperature sensor.");

    let mut i2c_mux_3 =
        I2cMux::new(&hyped_i2c, 2, DEFAULT_MUX_ADDRESS).expect("Failed to create I2C multiplexer.");
    let mut temp_3 = Temperature::new(&mut i2c_mux_3, TemperatureAddresses::Address3c)
        .expect("Failed to create temperature sensor.");

    let mut i2c_mux_4 =
        I2cMux::new(&hyped_i2c, 3, DEFAULT_MUX_ADDRESS).expect("Failed to create I2C multiplexer.");
    let mut temp_4 = Temperature::new(&mut i2c_mux_4, TemperatureAddresses::Address3c)
        .expect("Failed to create temperature sensor.");

    loop {
        match temp_1.read() {
            Some(temperature) => {
                defmt::info!("Temperature: {:?}", temperature);
            }
            None => {
                defmt::info!("Failed to read temperature.");
            }
        }

        match temp_2.read() {
            Some(temperature) => {
                defmt::info!("Temperature: {:?}", temperature);
            }
            None => {
                defmt::info!("Failed to read temperature.");
            }
        }

        match temp_3.read() {
            Some(temperature) => {
                defmt::info!("Temperature: {:?}", temperature);
            }
            None => {
                defmt::info!("Failed to read temperature.");
            }
        }

        match temp_4.read() {
            Some(temperature) => {
                defmt::info!("Temperature: {:?}", temperature);
            }
            None => {
                defmt::info!("Failed to read temperature.");
            }
        }
    }
}
