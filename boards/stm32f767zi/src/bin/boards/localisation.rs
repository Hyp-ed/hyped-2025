#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    init,
    spi::{self, BitOrder, Spi},
    time::khz,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use heapless::Vec;
use hyped_boards_stm32f767zi::{
    io::{Stm32f767ziGpioOutput, Stm32f767ziSpi},
    tasks::sensors::{read_keyence::read_keyence, read_optical_flow::read_optical_flow},
};
use hyped_core::config::{MeasurementId, LOCALISATION_CONFIG};
use hyped_localisation::{
    control::localizer::Localizer, preprocessing::optical, types::RawAccelerometerData,
};
use hyped_spi::HypedSpiCsPin;
use panic_probe as _;

/// A Watch to hold the latest Keyence stripe count
static KEYENCE_1_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();
static KEYENCE_2_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();

/// A Watch to hold the latest optical flow data
static OPTICAL_FLOW_DATA: Watch<
    CriticalSectionRawMutex,
    Vec<f64, { LOCALISATION_CONFIG.optical_flow.num_sensors as usize }>,
    1,
> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    // Import `init` so that we can initialize board peripherals.
    let p = init(Default::default());

    let mut spi_config = spi::Config::default();
    spi_config.frequency = khz(400);
    spi_config.bit_order = BitOrder::MsbFirst;

    let spi = Spi::new_blocking(p.SPI1, p.PB3, p.PB5, p.PB4, spi_config);
    let hyped_spi = Stm32f767ziSpi::new(spi);

    let cs = HypedSpiCsPin::new(Stm32f767ziGpioOutput::new(Output::new(
        p.PA4,
        Level::High,
        Speed::VeryHigh,
    )));

    spawner
        .spawn(read_optical_flow(hyped_spi, cs, OPTICAL_FLOW_DATA.sender()))
        .unwrap();

    spawner
        .spawn(read_keyence(
            Input::new(p.PC13, Pull::Down),
            MeasurementId::Keyence1,
            KEYENCE_1_STRIPE_COUNT.sender(),
        ))
        .unwrap();
    spawner
        .spawn(read_keyence(
            Input::new(p.PC14, Pull::Down),
            MeasurementId::Keyence2,
            KEYENCE_2_STRIPE_COUNT.sender(),
        ))
        .unwrap();

    // Initialise receivers
    let mut keyence_1_receiver = KEYENCE_1_STRIPE_COUNT.receiver().unwrap();
    let mut keyence_2_receiver = KEYENCE_2_STRIPE_COUNT.receiver().unwrap();
    let mut optical_flow_receiver = OPTICAL_FLOW_DATA.receiver().unwrap();
    let mut accelerometers_receiver = 

    let mut localizer = Localizer::new();

    info!("Starting localizer loop...");

    loop {
        // Wait for new Keyence stripe count.
        let stripe_count1 = keyence_1_receiver.get().await;
        let stripe_count2 = keyence_2_receiver.get().await;

        defmt::info!(
            "New Keyence stripe counts: sensor1 = {}, sensor2 = {}",
            stripe_count1,
            stripe_count2
        );

        // Create the sensor data. (no accelerometer data)
        let keyence_data: Vec<u32, 2> = Vec::from_slice(&[stripe_count1, stripe_count2]).unwrap();
        let accelerometer_data: RawAccelerometerData<
            { LOCALISATION_CONFIG.accelerometers.num_sensors as usize },
            { LOCALISATION_CONFIG.num_axis as usize },
        > = RawAccelerometerData::from_slice(&[
            Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
            Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
            Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
            Vec::from_slice(&[0.0, 0.0, 9.81]).unwrap(),
        ])
        .unwrap();

        let optical_data = optical_flow_receiver.get().await;

        match localizer.iteration(optical_data, keyence_data, accelerometer_data) {
            Ok(()) => {
                defmt::info!(
                    "Iteration OK: displacement = {} m, velocity = {} m/s, acceleration = {} m/s**2",
                    localizer.displacement,
                    localizer.velocity,
                    localizer.acceleration
                );
            }
            Err(e) => {
                defmt::error!("Iteration error: {:?}", e);
            }
        }

        Timer::after(Duration::from_millis(100)).await;
    }
}
