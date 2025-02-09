#![no_std]
#![no_main]


use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use crate::{
    Localizer,
    types::{RawAccelerometerData, NUM_ACCELEROMETERS, NUM_AXIS},
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Starting localizer loop...");

    let mut localizer = Localizer::new();

    loop {

        //Simulated data

        let optical_data: Vec<f64, 2> = Vec::from_slice(&[0.5, 0.5]).unwrap();

        let keyence_data: Vec<u32, 2> = Vec::from_slice(&[1, 1]).unwrap();

        let accelerometer_data: RawAccelerometerData<NUM_ACCELEROMETERS, NUM_AXIS> =
            RawAccelerometerData::from_slice(&[
                Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
                Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
                Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
                Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
            ])
            .unwrap();

        match localizer.iteration(optical_data, keyence_data, accelerometer_data) {
            Ok(()) => {
                info!(
                    "Iteration OK: displacement = {} m, velocity = {} m/s, acceleration = {} m/s²",
                    localizer.displacement, localizer.velocity, localizer.acceleration
                );
            }
            Err(e) => {
                error!("Iteration error: {:?}", e);
            }
        }

        // Wait 100ms before the next update.
        Timer::after(Duration::from_millis(100)).await;
    }
}
