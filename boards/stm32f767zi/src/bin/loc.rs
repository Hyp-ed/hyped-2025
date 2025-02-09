#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{init, gpio::{Input, Pull}};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::tasks::read_keyence::read_keyence;
use panic_probe as _;

use heapless::Vec;

use hyped_localisation::{
    control::localizer::Localizer,
    types::{RawAccelerometerData, NUM_ACCELEROMETERS, NUM_AXIS},
};

/// A Watch to hold the latest Keyence stripe count.
static KEYENCE_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();


#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    // Import `init` so that we can initialize board peripherals.
    let p = init(Default::default());
    let gpio_pin = Input::new(p.PC13, Pull::Down);

    // Create a sender and a receiver for the Keyence stripe count.
    let sender = KEYENCE_STRIPE_COUNT.sender();
    let mut receiver = KEYENCE_STRIPE_COUNT.receiver().unwrap();

    spawner.spawn(read_keyence(gpio_pin, sender)).unwrap();

    info!("Starting localizer loop...");

    let mut localizer = Localizer::new();

    loop {
        // Wait for a new Keyence stripe count.
        let new_stripe_count = receiver.get().await;
        defmt::info!("New Keyence stripe count: {}", new_stripe_count);

        // Create the sensor data. (Optical and accelerometer data are simulated.)
        let optical_data: Vec<f64, 2> = Vec::from_slice(&[0.5, 0.5]).unwrap();
        let keyence_data: Vec<u32, 2> = Vec::from_slice(&[new_stripe_count, new_stripe_count]).unwrap();
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
                defmt::info!(
                    "Iteration OK: displacement = {} m, velocity = {} m/s, acceleration = {} m/s**2",
                    localizer.displacement(),
                    localizer.velocity(),
                    localizer.acceleration()
                );
            }
            Err(e) => {
                defmt::error!("Iteration error: {:?}", e);
            }
        }

        Timer::after(Duration::from_millis(100)).await;
    }
}