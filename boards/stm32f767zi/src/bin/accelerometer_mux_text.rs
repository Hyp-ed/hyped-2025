#![no_main]
#![no_std]

use core::cell::RefCell;

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{i2c::I2c, mode::Blocking, time::Hertz};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use heapless::Vec;
use hyped_boards_stm32f767zi::tasks::sensors::read_accelerometer_from_mux::{
    read_accelerometer_from_mux, AccelerometerMuxReadings, MuxAddressChannelPair,
};
use hyped_core::config::LOCALISATION_CONFIG;
use hyped_sensors::SensorValueRange::*;
use panic_probe as _;
use static_cell::StaticCell;
const NUM_ACCELEROMETERS: usize = LOCALISATION_CONFIG.accelerometers.num_sensors as usize;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

static ACCELERATION_MUX_READINGS: Watch<CriticalSectionRawMutex, AccelerometerMuxReadings, 1> =
    Watch::new();

const MUX_ADDRESS_CHANNEL_PAIRS: Vec<MuxAddressChannelPair, NUM_ACCELEROMETERS> = Vec::new(); // addresses and channels will be determined later

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(200_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the task.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));
    defmt::info!("I2C initialized.");

    let accelerometer_mux_reading_sender = ACCELERATION_MUX_READINGS.sender();
    let mut accelerometer_mux_reading_receiver = ACCELERATION_MUX_READINGS.receiver().unwrap();

    // Spawn the mux reading task
    spawner
        .spawn(read_accelerometer_from_mux(
            i2c_bus,
            MUX_ADDRESS_CHANNEL_PAIRS,
            accelerometer_mux_reading_sender,
        ))
        .unwrap();

    loop {
        if let Some(readings) = accelerometer_mux_reading_receiver.try_changed() {
            for (i, reading) in readings.iter().enumerate() {
                match reading {
                    Some(reading) => match reading {
                        Safe(accelerometer_values) => {
                            defmt::info!(
                                "Accelerometer {} reading: x={:?}mg, y={:?}mg, z={:?}mg (safe)",
                                i,
                                accelerometer_values.x,
                                accelerometer_values.y,
                                accelerometer_values.z
                            );
                        }
                        Warning(accelerometer_values) => {
                            defmt::info!(
                                "Accelerometer {} reading: x={:?}mg, y={:?}mg, z={:?}mg (unsafe)",
                                i,
                                accelerometer_values.x,
                                accelerometer_values.y,
                                accelerometer_values.z
                            );
                        }
                        Critical(accelerometer_values) => {
                            defmt::info!(
                                "Accelerometer {} reading: x={:?}mg, y={:?}mg, z={:?}mg (critical)",
                                i,
                                accelerometer_values.x,
                                accelerometer_values.y,
                                accelerometer_values.z
                            );
                        }
                    },
                    None => {
                        defmt::info!("Accelerometer {} reading: None", i);
                    }
                }
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
