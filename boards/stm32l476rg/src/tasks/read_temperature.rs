use crate::io::Stm32l476rgI2c;
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Sender,
};
use embassy_time::{Duration, Timer};
use hyped_sensors::{
    temperature::{Status, Temperature, TemperatureAddresses},
    SensorValueRange,
};

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// The update frequency of the temperature sensor in Hz
const UPDATE_FREQUENCY: u64 = 1000;

/// Test task that just reads the temperature from the sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_temperature(
    i2c_bus: &'static I2c1Bus,
    sender: Sender<'static, CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1>,
) -> ! {
    let mut hyped_i2c = Stm32l476rgI2c::new(i2c_bus);

    let mut temperature_sensor = Temperature::new(&mut hyped_i2c, TemperatureAddresses::Address3f)
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

        sender.send(temperature_sensor.read());
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
