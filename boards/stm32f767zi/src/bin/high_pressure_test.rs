#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use hyped_boards_stm32f767zi::tasks::read_high_pressure::read_high_pressure;
use hyped_sensors::high_pressure::{HighPressureError, State};
use {defmt_rtt as _, panic_probe as _};

/// used to store latest high pressure sensor value
static HIGH_PRESSURE_SENSOR_VALUE: Watch<
    CriticalSectionRawMutex,
    Result<State, HighPressureError>,
    1
> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let gpio1 = Input::new(p.PC12, Pull::Down);
    let gpio2 = Input::new(p.PC13, Pull::Down);

    // Create a sender to pass to the high pressure reading task, and a receiver for reading the values back.
    let sender = HIGH_PRESSURE_SENSOR_VALUE.sender();
    let mut receiver = HIGH_PRESSURE_SENSOR_VALUE.receiver().unwrap();

    spawner
        .spawn(read_high_pressure(gpio1, gpio2, sender))
        .unwrap();

    // only prints when high pressure value updates
    loop {
        let new_high_pressure_value = receiver.get().await;
        defmt::info!("High pressure sensor value: {}", new_high_pressure_value);
    }
}
