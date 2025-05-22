#![no_std]
#![no_main]

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
use hyped_boards_stm32l476rg::tasks::read_temperature::read_temperature;
use hyped_sensors::SensorValueRange::{self, *};
use panic_probe as _;
use static_cell::StaticCell;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Used to keep the latest temperature sensor value.
static TEMP_READING: Watch<CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the tasks.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));

    // Create a sender to pass to the temperature reading task, and a receiver for reading the values back.
    let temp_reading_sender = TEMP_READING.sender();
    let mut temp_reading_receiver = TEMP_READING.receiver().unwrap();

    spawner.must_spawn(read_temperature(i2c_bus, temp_reading_sender));

    // Every 100ms we read for the latest value from the temperature sensor.
    loop {
        if let Some(reading) = temp_reading_receiver.try_changed() {
            match reading {
                Some(reading) => match reading {
                    Safe(temp) => {
                        defmt::info!("Temperature: {}°C (safe)", temp);
                    }
                    Warning(temp) => {
                        defmt::warn!("Temperature: {}°C (warning)", temp);
                    }
                    Critical(temp) => {
                        defmt::error!("Temperature: {}°C (critical)", temp);
                    }
                },
                None => defmt::warn!("No temperature reading available."),
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
