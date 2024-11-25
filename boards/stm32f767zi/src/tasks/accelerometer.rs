use crate::io::i2c::Stm32f767ziI2c;
use defmt_rtt as _;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use hyped_sensors::accelerometer::{
    AccelerationValues, Accelerometer, AccelerometerAddresses, Status,
};

/// Test task that reads the acceleration from the sensor and prints it to the console.
#[embassy_executor::task]
pub async fn read_acceleration() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(200_000), Default::default());
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
            Some(accel_values) => {
                defmt::info!(
                    "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg",
                    accel_values.x,
                    accel_values.y,
                    accel_values.z
                );
            }
            None => {
                defmt::info!("Failed to read acceleration values.")
            }
        }
    }
}
