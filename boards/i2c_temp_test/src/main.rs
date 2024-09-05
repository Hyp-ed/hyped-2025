#![no_std]
#![no_main]

use core::panic;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Timer};
use hyped_io_stm32l476rg::i2c::Stm32l476rgGpioI2c;
use hyped_sensors::temperature::{Status, Temperature};
use {defmt_rtt as _, panic_probe as _};

/// Test task that just reads the temperature from the sensor and prints it to the console
#[embassy_executor::task]
async fn temp() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());
    let hyped_i2c = Stm32l476rgGpioI2c::new(i2c);

    let mut temperature_sensor = Temperature::new(hyped_i2c).expect(
        "Failed to create temperature sensor. Check the wiring and the I2C address of the sensor.",
    );

    loop {
        match temperature_sensor.check_status() {
            Status::TempOverUpperLimit => {
                defmt::error!("Temperature is over the upper limit.");
            }
            Status::TempUnderLowerLimit => {
                defmt::error!("Temperature is under the lower limit.");
            }
            Status::Busy => {
                defmt::warn!("Temperature sensor is busy.");
            }
            Status::Unknown => {
                panic!("Could not get the status of the temperature sensor.")
            }
            Status::Ok => {}
        }

        match temperature_sensor.read() {
            Some(temperature) => {
                defmt::info!("Temperature: {:?}", temperature);
            }
            None => {
                defmt::info!("Failed to read temperature.");
            }
        }
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
