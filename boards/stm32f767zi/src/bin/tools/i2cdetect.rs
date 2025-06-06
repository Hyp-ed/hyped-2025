#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{i2c::I2c, mode::Blocking, time::Hertz};

use panic_probe as _;

/// Rust/Embassy version of the i2cdetect tool.
/// This tool scans the I2C bus for devices and prints their addresses.
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    defmt::debug!("i2cdetect");

    // Initialize the I2C bus
    let p = embassy_stm32::init(Default::default());
    let mut i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());

    defmt::info!("Starting I2C scan...");
    i2cdetect_scan(&mut i2c);
    defmt::info!("I2C scan complete.");

    // Don't do anything else, just loop forever
    // Could modify this to loop calling i2cdetect_scan periodically
    loop {}
}

fn i2cdetect_scan(i2c: &mut I2c<Blocking>) {
    for address in 0x03..=0x77 {
        if let Ok(_) = i2c.blocking_read(address, &mut [0; 1]) {
            defmt::info!("Found device at address: 0x{:02X}", address);
        }
    }
}
