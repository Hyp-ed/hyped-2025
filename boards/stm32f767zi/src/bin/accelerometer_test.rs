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
use hyped_boards_stm32f767zi::tasks::sensors::read_accelerometer::read_accelerometer;
use hyped_sensors::{
    accelerometer::AccelerationValues,
    SensorValueRange::{self, *},
};
use panic_probe as _;
use static_cell::StaticCell;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Used to keep the latest acceleration values.
static ACCELEROMETER_READING: Watch<
    CriticalSectionRawMutex,
    Option<SensorValueRange<AccelerationValues>>,
    1,
> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(200_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the tasks.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));

    // Create a sender to pass to the acceleration reading task, and a receiver for reading the values back.
    let accelerometer_reading_sender = ACCELEROMETER_READING.sender();
    let mut accelerometer_reading_receiver = ACCELEROMETER_READING.receiver().unwrap();

    spawner.must_spawn(read_accelerometer(i2c_bus, accelerometer_reading_sender));

    // Every 100ms we read for the latest value from the accelerometer.
    loop {
        if let Some(accelerometer_values) = accelerometer_reading_receiver.try_changed() {
            match accelerometer_values {
                Some(accelerometer_values) => match accelerometer_values {
                    Safe(accelerometer_values) => {
                        defmt::info!(
                            "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (safe)",
                            accelerometer_values.x,
                            accelerometer_values.y,
                            accelerometer_values.z
                        );
                    }
                    Warning(accelerometer_values) => {
                        defmt::info!(
                            "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (unsafe)",
                            accelerometer_values.x,
                            accelerometer_values.y,
                            accelerometer_values.z
                        );
                    }
                    Critical(accelerometer_values) => {
                        defmt::info!(
                            "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (critical)",
                            accelerometer_values.x,
                            accelerometer_values.y,
                            accelerometer_values.z
                        );
                    }
                },
                None => {
                    defmt::info!("Failed to read acceleration values.")
                }
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
