#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Output, Level, Pull, Speed},
    init,
    spi::{self, BitOrder, Spi},
    time::khz,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use heapless::Vec;
use hyped_boards_stm32f767zi::{
    io::{Stm32f767ziGpioOutput, Stm32f767ziSpi},
    tasks::read_keyence::read_keyence,
};
use hyped_localisation::{
    control::localizer::Localizer,
    types::{RawAccelerometerData, NUM_ACCELEROMETERS, NUM_AXIS},
};
use hyped_sensors::optical_flow::OpticalFlow;
use hyped_spi::HypedSpiCsPin;
use panic_probe as _;

/// A Watch to hold the latest Keyence stripe count.
static KEYENCE_1_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();
static KEYENCE_2_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    // Import `init` so that we can initialize board peripherals.
    let p = init(Default::default());

    let mut spi_config = spi::Config::default();
    spi_config.frequency = khz(400);
    spi_config.bit_order = BitOrder::MsbFirst;

    let spi = Spi::new_blocking(p.SPI1, p.PB3, p.PB5, p.PB4, spi_config);
    let mut hyped_spi = Stm32f767ziSpi::new(spi);

    let cs = HypedSpiCsPin::new(Stm32f767ziGpioOutput::new(Output::new(
        p.PA4,
        Level::High,
        Speed::VeryHigh,
    )));

    let mut optical_flow = OpticalFlow::new(&mut hyped_spi, cs)
        .await
        .expect("Failed to initialize optical flow sensor.");

    defmt::info!("Optical flow sensor initialized.");

    let gpio_pin1 = Input::new(p.PC13, Pull::Down);
    let gpio_pin2 = Input::new(p.PC14, Pull::Down);

    // Create a sender and a receiver for the Keyence stripe count.
    let sender1 = KEYENCE_1_STRIPE_COUNT.sender();
    let mut receiver1 = KEYENCE_1_STRIPE_COUNT.receiver().unwrap();
    let sender2 = KEYENCE_2_STRIPE_COUNT.sender();
    let mut receiver2 = KEYENCE_2_STRIPE_COUNT.receiver().unwrap();

    spawner.spawn(read_keyence(gpio_pin1, sender1)).unwrap();
    spawner.spawn(read_keyence(gpio_pin2, sender2)).unwrap();

    info!("Starting localizer loop...");

    let mut localizer = Localizer::new();

    loop {
        // Wait for a new Keyence stripe count.
        let stripe_count1 = receiver1.get().await;
        let stripe_count2 = receiver2.get().await;

        let flow = optical_flow.get_motion().await.unwrap();
        let optical_data: Vec<f64, 2> = Vec::from_slice(&[flow.x as f64, flow.y as f64])
            .unwrap();

        defmt::info!(
            "New Keyence stripe counts: sensor1 = {}, sensor2 = {}",
            stripe_count1,
            stripe_count2
        );

        // Create the sensor data. (accelerometer data is simulated.)
        let keyence_data: Vec<u32, 2> = Vec::from_slice(&[stripe_count1, stripe_count2]).unwrap();
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
