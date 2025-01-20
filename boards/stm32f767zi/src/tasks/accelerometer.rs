use crate::io::Stm32f767ziI2c;
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::time::Hertz;
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use hyped_sensors::{
    accelerometer::{Accelerometer, AccelerometerAddresses, Status},
    SensorValueRange::*,
};
use static_cell::StaticCell;
type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Test task that reads the acceleration from the sensor and prints it to the console.
#[embassy_executor::task]
pub async fn read_acceleration() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(200_000), Default::default());
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));
    let mut hyped_i2c = Stm32f767ziI2c::new(i2c);

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

        match accelerometer.read() {
            Some(accel_values) => match accel_values {
                Safe(accel_values) => {
                    defmt::info!(
                        "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (safe)",
                        accel_values.x,
                        accel_values.y,
                        accel_values.z
                    );
                }
                Warning(accel_values) => {
                    defmt::info!(
                        "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (unsafe)",
                        accel_values.x,
                        accel_values.y,
                        accel_values.z
                    );
                }
                Critical(accel_values) => {
                    defmt::info!(
                        "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (critical)",
                        accel_values.x,
                        accel_values.y,
                        accel_values.z
                    );
                }
            },
            None => {
                defmt::info!("Failed to read acceleration values.")
            }
        }
    }
}
