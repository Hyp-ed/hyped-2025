use crate::io::Stm32l476rgI2c;
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_sync::blocking_mutex::Mutex;
use hyped_core::types::SensorValueBounds::{Critical, Safe, Warning};
use hyped_sensors::temperature::{Status, Temperature, TemperatureAddresses};

/// Test task that just reads the temperature from the sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_temp() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = Mutex::new(RefCell::new(I2c::new_blocking(
        p.I2C1,
        p.PB8,
        p.PB9,
        Hertz(100_000),
        Default::default(),
    )));
    let mut hyped_i2c = Stm32l476rgI2c::new(i2c);

    let mut temperature_sensor = Temperature::new(
        &mut hyped_i2c,
        TemperatureAddresses::Address3f,
        None,
    )
    .expect(
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
            Some(temperature) => match temperature {
                Safe(temp) => {
                    defmt::info!("Temperature: {}°C (safe)", temp);
                }
                Warning(temp) => {
                    defmt::warn!("Temperature: {}°C (warning)", temp);
                }
                Critical(temp) => {
                    defmt::error!("Temperature: {}°C (critical)", temp);
                }
            },
            None => {
                defmt::info!("Failed to read temperature.");
            }
        }
    }
}
