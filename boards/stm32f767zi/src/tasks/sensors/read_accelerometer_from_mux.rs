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
use hyped_core::config::LOCALISATION_CONFIG;
use hyped_i2c::i2c_mux::I2cMux;
use hyped_sensors::{
    accelerometer::{self, AccelerationValues, Accelerometer, AccelerometerAddresses},
    SensorValueRange::{self},
};
const NUM_ACCELEROMETERS: usize = LOCALISATION_CONFIG.accelerometers.num_sensors as usize;

pub struct MuxAddressChannelPair {
    pub address: u8,
    pub channel: u8,
}

pub type AccelerometerMuxReadings =
    Vec<Option<SensorValueRange<AccelerationValues>>, NUM_ACCELEROMETERS>;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Task that reads the accelerometers on the muxes given in `mux_address_channel_pairs`
#[embassy_executor::task]
pub async fn read_accelerometer_from_mux(
    i2c_bus: &'static I2c1Bus,
    mut mux_address_channel_pairs: Vec<MuxAddressChannelPair, NUM_ACCELEROMETERS>,
    sender: Sender<'static, CriticalSectionRawMutex, AccelerometerMuxReadings, 1>,
) -> ! {
    let mut i2c_muxes: Vec<I2cMux<Stm32f767ziI2c<'_>>, NUM_ACCELEROMETERS> =
        mux_address_channel_pairs
            .iter_mut()
            .map(|pair| {
                let hyped_i2c = Stm32f767ziI2c::new(i2c_bus);
                let i2c_mux = match I2cMux::new(hyped_i2c, pair.channel, pair.address) {
                    Ok(i2c_mux) => i2c_mux,
                    Err(_) => panic!(
                        "Failed to create I2C Mux on address {} and channel {}.
                     Check the wiring and address of the Mux",
                        pair.address, pair.channel
                    ),
                };

                i2c_mux
            })
            .collect();

    // Create a vector of accelerometers using the vector of muxes.
    let mut accelerometers : Vec<Option<Accelerometer<'_, I2cMux<Stm32f767ziI2c<'_>>>>, NUM_ACCELEROMETERS> =
     i2c_muxes.iter_mut()
        .map(|i2c_mux| {
        match Accelerometer::new(i2c_mux, AccelerometerAddresses::Address1d) {
            Ok(accelerometer) => {
                defmt::info!("Accelerometer created.");
                Some(accelerometer)
            }
            Err(_) => {
                defmt::info!("Failed to create accelerometer. Check the wiring and the I2C address of the accelerometer.");
                None
            }}}).collect();

    loop {
        let mut readings: AccelerometerMuxReadings = Vec::new();

        // Read from all accelerometers
        for i in 0..NUM_ACCELEROMETERS {
            let accelerometer = &mut accelerometers[i];

            match accelerometer {
                Some(accelerometer) => {
                    match accelerometer.check_status() {
                        accelerometer::Status::Ok => {}
                        accelerometer::Status::DataNotReady => {
                            defmt::warn!("Accelerometer is not ready to provide data")
                        }
                        accelerometer::Status::Unknown => {
                            panic!("Could not get status of accelerometer")
                        }
                    }

                    readings
                        .push(accelerometer.read())
                        .expect("Failed to add acceleration reading to vector of readings.");
                }
                None => readings
                    .push(None)
                    .expect("Failed to add None to vector of readings"),
            }
        }

        sender.send(readings);
        Timer::after(Duration::from_millis(100)).await;
    }
}
