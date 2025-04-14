use crate::io::Stm32f767ziI2c;
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::blocking_mutex::{
    raw::{CriticalSectionRawMutex, NoopRawMutex},
    Mutex,
};
use embassy_sync::watch::Sender;
use embassy_time::{Duration, Timer};
use hyped_sensors::{
    accelerometer::{AccelerationValues, Accelerometer, AccelerometerAddresses, Status},
    SensorValueRange,
};

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Update frequency of accelerometer in Hz
const UPDATE_FREQUENCY: u64 = 200;

/// Test task that reads the acceleration from the sensor and prints it to the console.
#[embassy_executor::task]
pub async fn read_accelerometer(
    i2c_bus: &'static I2c1Bus,
    sender: Sender<
        'static,
        CriticalSectionRawMutex,
        Option<SensorValueRange<AccelerationValues>>,
        1,
    >,
) -> ! {
    let mut hyped_i2c = Stm32f767ziI2c::new(i2c_bus);

    let mut accelerometer = Accelerometer::new(&mut hyped_i2c, AccelerometerAddresses::Address1d)
        .expect(
            "Failed to create accelerometer. Check the wiring and the I2C address of the sensor.",
        );

    loop {
        match accelerometer.check_status() {
            Status::DataNotReady => {
                defmt::warn!("Accelerometer is not ready to provide data");
            }
            Status::Unknown => {
                panic!("Could not get the status of the accelerometer.");
            }
            Status::Ok => {}
        }

        sender.send(accelerometer.read());
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
