#![no_std]
#![no_main]

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
use hyped_boards_stm32l432kc::tasks::laser_triangulation::read_laser_triangulation_distance;
use hyped_sensors::{laser_triangulation::LaserTriangulation, SensorValueRange::*};
use {defmt_rtt as _, panic_probe as _};

static LASER_TRIANGULATION_READING: Watch<CriticalSectionRawMutex, SensorValueRange<f32>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1, Delay);

    // Create a sender to pass to the laser triangulation sensor reading task, and a receiver for reading the values back.
    let laser_triangulation_reading_sender = LASER_TRIANGULATION_READING.sender();
    let mut laser_triangulation_reading_receiver = LASER_TRIANGULATION_READING.receiver().unwrap();

    spawner
        .spawn(read_laser_triangulation_distance(laser_triangulation_reading_sender))
        .unwrap();

    // Every 100ms we read for the latest value from the laser triangulation sensor.
    loop {
        match laser_triangulation_reading_receiver.try_changed() {
            Some(reading) => match reading {
                Safe(value) => {
                    defmt::info!("Range: {} mm (safe)", value)
                }
                Warning(value) => {
                    defmt::warning!("Range: {} mm (warning)", value)
                }
                Critical(value) => {
                    defmt::error!("Range: {} A (critical)", value)
                }
                None => (),
            }
    }
    Timer::after(Duration::from_millis(100)).await;
    }
}