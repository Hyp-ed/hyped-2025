#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l432kc::tasks::read_current_levitation::read_current_levitation;
use hyped_sensors::SensorValueRange;
use hyped_sensors::SensorValueRange::*;
use {defmt_rtt as _, panic_probe as _};

static CURRENT_LEVITATION_READING: Watch<CriticalSectionRawMutex, SensorValueRange<f32>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Create a sender to pass to the current levitation reading task, and a receiver for reading the values back.
    let current_levitation_reading_sender = CURRENT_LEVITATION_READING.sender();
    let mut current_levitation_reading_receiver = CURRENT_LEVITATION_READING.receiver().unwrap();

    spawner
        .spawn(read_current_levitation(current_levitation_reading_sender))
        .unwrap();

    // Every 100ms we read for the latest value from the current levitation sensor.
    loop {
        match current_levitation_reading_receiver.try_changed() {
            Some(reading) => match reading {
                Safe(value) => {
                    defmt::info!("Current: {} A (safe)", value)
                }
                Warning(value) => {
                    defmt::warn!("Current: {} A (warning)", value)
                }
                Critical(value) => {
                    defmt::error!("Current: {} A (critical)", value)
                }
            },
            None => (),
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}
