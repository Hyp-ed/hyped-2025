#![no_std]
#![no_main]

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_stm32::adc::Adc;
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l432kc::tasks::current_levitation::read_current_levitation;
use hyped_sensors::{current_levitation::CurrentLevitation, SensorValueRange::*};
use {defmt_rtt as _, panic_probe as _};

static CURRENT_LEVITATION_READING: Watch<CriticalSectionRawMutex, SensorValueRange<f32>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1, Delay);

    let current_reading_sender = CURRENT_LEVITATION_READING.sender();
    let mut current_lev_reading_receiver = CURRENT_LEVITATION_READING.receiver().unwrap();

    // Create a sender to pass to the current levitation reading task, and a receiver for reading the values back.
    spawner
        .spawn(read_current_levitation(current_reading_sender))
        .unwrap();

    // Every 100ms we read for the latest value from the current levitation sensor.
    loop {
        match current_lev_reading_receiver.try_changed() {
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
                None => (),
            }
    }
    Timer::after(Duration::from_millis(100)).await;
    }
}