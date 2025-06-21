#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    i2c::I2c,
    init,
    mode::Blocking,
    spi::{self, BitOrder, Spi},
    time::{khz, Hertz},
};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use heapless::Vec;
use hyped_boards_stm32f767zi::{
    io::{Stm32f767ziGpioOutput, Stm32f767ziSpi},
    tasks::sensors::{
        read_accelerometers_from_mux::{read_accelerometers_from_mux, AccelerometerMuxReadings},
        read_keyence::read_keyence,
        read_optical_flow::read_optical_flow,
    },
};
use hyped_core::config::{MeasurementId, LOCALISATION_CONFIG};
use hyped_localisation::{control::localizer::Localizer, types::RawAccelerometerData};
use hyped_spi::HypedSpiCsPin;
use panic_probe as _;
use static_cell::StaticCell;
type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// A Watch to hold the latest Keyence stripe count
static KEYENCE_1_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();
static KEYENCE_2_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();

/// A Watch to hold the latest optical flow data
static OPTICAL_FLOW_DATA: Watch<CriticalSectionRawMutex, Vec<f64, 2>, 1> = Watch::new();

/// A Watch to hold the latest accelerometer data
static ACCELEROMETERS_DATA: Watch<CriticalSectionRawMutex, AccelerometerMuxReadings, 1> =
    Watch::new();

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

    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(200_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the task.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));
    defmt::info!("I2C initialized.");

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

    spawner
        .spawn(read_accelerometers_from_mux(
            i2c_bus,
            ACCELEROMETERS_DATA.sender(),
        ))
        .unwrap();

    // Initialise receivers
    let mut keyence_1_receiver = KEYENCE_1_STRIPE_COUNT.receiver().unwrap();
    let mut keyence_2_receiver = KEYENCE_2_STRIPE_COUNT.receiver().unwrap();
    let mut optical_flow_receiver = OPTICAL_FLOW_DATA.receiver().unwrap();
    let mut accelerometers_receiver = ACCELEROMETERS_DATA.receiver().unwrap();

    let mut localizer = Localizer::new();

    info!("Starting localizer loop...");

    loop {
        let keyence_data: Vec<u32, 2> = Vec::from_slice(&[
            keyence_1_receiver.get().await,
            keyence_2_receiver.get().await,
        ])
        .unwrap();

        let accelerometer_data: RawAccelerometerData<
            { LOCALISATION_CONFIG.accelerometers.num_sensors as usize },
            { LOCALISATION_CONFIG.num_axis as usize },
        > = accelerometers_receiver.get().await;

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
