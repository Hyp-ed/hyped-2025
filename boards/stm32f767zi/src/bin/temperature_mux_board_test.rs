#![no_std]
#![no_main]

use core::cell::RefCell;

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
use hyped_boards_stm32f767zi::tasks::read_temperature_mux_board::{
    self, read_temperature_mux_board, TemperatureMuxReadings,
};
use hyped_sensors::SensorValueRange::{self, *};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Used to keep the latest temperature sensor value.
static TEMP_READINGS: Watch<CriticalSectionRawMutex, TemperatureMuxReadings, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the tasks.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));

    // Create a sender to pass to the temperature reading task, and a receiver for reading the values back.
    let temp_reading_sender = TEMP_READINGS.sender();
    let mut temp_reading_receiver = TEMP_READINGS.receiver().unwrap();

    spawner
        .spawn(read_temperature_mux_board(i2c_bus, temp_reading_sender))
        .unwrap();

    // Every 100ms we read for the latest value from the temperature sensor.
    loop {
        match temp_reading_receiver.try_changed() {
            Some(readings) => {
                for (i, reading) in readings.iter().enumerate() {
                    match reading {
                        Some(reading) => match reading {
                            SensorValueRange::Critical(value) => {
                                defmt::info!("Temperature sensor {} reading: {}", i, value);
                            }
                            SensorValueRange::Warning(value) => {
                                defmt::info!("Temperature sensor {} reading: {}", i, value);
                            }
                            SensorValueRange::Safe(value) => {
                                defmt::info!("Temperature sensor {} reading: {}", i, value);
                            }
                        },
                        None => {
                            defmt::info!("Temperature sensor {} reading: None", i);
                        }
                    }
                }
            }
            None => (),
        }
        Timer::after(Duration::from_millis(1000)).await;
    }
}
