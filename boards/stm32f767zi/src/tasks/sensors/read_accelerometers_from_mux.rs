use crate::io::Stm32f767ziI2c;
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
use heapless::Vec;
use hyped_core::config::{LOCALISATION_CONFIG, SENSORS_CONFIG};
use hyped_i2c::{i2c_mux::DEFAULT_MUX_ADDRESS, HypedI2c};
use hyped_sensors::{
    accelerometer::{self, AccelerationValues, Accelerometer, AccelerometerAddresses},
    SensorValueRange::{self},
};
const NUM_ACCELEROMETERS: usize = LOCALISATION_CONFIG.accelerometers.num_sensors as usize;

pub type AccelerometerMuxReadings =
    Vec<Option<SensorValueRange<AccelerationValues>>, NUM_ACCELEROMETERS>;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Task that reads the accelerometers on the muxes given in `mux_address_channel_pairs`
#[embassy_executor::task]
pub async fn read_accelerometers_from_mux(
    i2c_bus: &'static I2c1Bus,
    sender: Sender<'static, CriticalSectionRawMutex, AccelerometerMuxReadings, 1>,
) -> ! {
    let mut hyped_i2c = Stm32f767ziI2c::new(i2c_bus);

    // Select channel 0
    hyped_i2c
        .write_byte(DEFAULT_MUX_ADDRESS, 1 << 0)
        .expect("Failed to select channel 0 on mux");
    defmt::info!("Mux initialized and channel 0 selected.");

    let mut i2c_for_accelerometer_1 = Stm32f767ziI2c::new(i2c_bus);
    let mut accelerometer_1 = Accelerometer::new(
        &mut i2c_for_accelerometer_1,
        AccelerometerAddresses::Address1d,
    )
    .expect("Failed to create accelerometer. Check the wiring and the I2C address of the sensor.");
    defmt::info!("Accelerometer 1 initialized.");

    let mut i2c_for_accelerometer_2 = Stm32f767ziI2c::new(i2c_bus);
    let mut accelerometer_2 = Accelerometer::new(
        &mut i2c_for_accelerometer_2,
        AccelerometerAddresses::Address1e,
    )
    .expect("Failed to create accelerometer. Check the wiring and the I2C address of the sensor.");
    defmt::info!("Accelerometer 2 initialized.");

    // Select channel 1
    hyped_i2c
        .write_byte(DEFAULT_MUX_ADDRESS, 1 << 1)
        .expect("Failed to select channel 1 on mux");
    defmt::info!("Channel 1 selected on mux.");

    let mut i2c_for_accelerometer_3 = Stm32f767ziI2c::new(i2c_bus);
    let mut accelerometer_3 = Accelerometer::new(
        &mut i2c_for_accelerometer_3,
        AccelerometerAddresses::Address1d,
    )
    .expect("Failed to create accelerometer. Check the wiring and the I2C address of the sensor.");
    defmt::info!("Accelerometer 3 initialized.");

    let mut i2c_for_accelerometer_4 = Stm32f767ziI2c::new(i2c_bus);
    let mut accelerometer_4 = Accelerometer::new(
        &mut i2c_for_accelerometer_4,
        AccelerometerAddresses::Address1e,
    )
    .expect("Failed to create accelerometer. Check the wiring and the I2C address of the sensor.");
    defmt::info!("Accelerometer 4 initialized.");

    loop {
        let mut readings: AccelerometerMuxReadings = Vec::new();

        // Read from all accelerometers
        defmt::info!("Reading accelerometers from mux");

        // Select channel 0
        hyped_i2c
            .write_byte(DEFAULT_MUX_ADDRESS, 1 << 0)
            .expect("Failed to select channel 0 on mux");

        // Read the first accelerometer
        match accelerometer_1.check_status() {
            accelerometer::Status::Ok => {}
            accelerometer::Status::DataNotReady => {
                defmt::warn!("Accelerometer is not ready to provide data")
            }
            accelerometer::Status::Unknown => {
                panic!("Could not get status of accelerometer")
            }
        }

        readings
            .push(accelerometer_1.read())
            .expect("Failed to add acceleration reading to vector of readings.");

        // Read the second accelerometer
        match accelerometer_2.check_status() {
            accelerometer::Status::Ok => {}
            accelerometer::Status::DataNotReady => {
                defmt::warn!("Accelerometer is not ready to provide data")
            }
            accelerometer::Status::Unknown => {
                panic!("Could not get status of accelerometer")
            }
        }

        readings
            .push(accelerometer_2.read())
            .expect("Failed to add acceleration reading to vector of readings.");

        // Select channel 1
        hyped_i2c
            .write_byte(DEFAULT_MUX_ADDRESS, 1 << 1)
            .expect("Failed to select channel 1 on mux");

        // Read the first accelerometer
        match accelerometer_3.check_status() {
            accelerometer::Status::Ok => {}
            accelerometer::Status::DataNotReady => {
                defmt::warn!("Accelerometer is not ready to provide data")
            }
            accelerometer::Status::Unknown => {
                panic!("Could not get status of accelerometer")
            }
        }
        readings
            .push(accelerometer_3.read())
            .expect("Failed to add acceleration reading to vector of readings.");

        // Read the second accelerometer
        match accelerometer_4.check_status() {
            accelerometer::Status::Ok => {}
            accelerometer::Status::DataNotReady => {
                defmt::warn!("Accelerometer is not ready to provide data")
            }
            accelerometer::Status::Unknown => {
                panic!("Could not get status of accelerometer")
            }
        }
        readings
            .push(accelerometer_4.read())
            .expect("Failed to add acceleration reading to vector of readings.");

        sender.send(readings);
        Timer::after(Duration::from_hz(
            SENSORS_CONFIG.sensors.accelerometer.update_frequency as u64,
        ))
        .await;
    }
}
