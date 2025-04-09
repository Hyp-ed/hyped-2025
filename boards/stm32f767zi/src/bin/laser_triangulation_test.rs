#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use hyped_boards_stm32f767zi::tasks::read_laser_triangulation::read_laser_triangulation;
use hyped_sensors::SensorValueRange;
use hyped_sensors::SensorValueRange::*;
use {defmt_rtt as _, panic_probe as _};

static LASER_TRIANGULATION_READING: Watch<CriticalSectionRawMutex, SensorValueRange<f32>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Create a sender to pass to the laser triangulation sensor reading task, and a receiver for reading the values back.
    let laser_triangulation_reading_sender = LASER_TRIANGULATION_READING.sender();
    let mut laser_triangulation_reading_receiver = LASER_TRIANGULATION_READING.receiver().unwrap();

    spawner.must_spawn(read_laser_triangulation(laser_triangulation_reading_sender));
    // Every 100ms we read for the latest value from the laser triangulation sensor.
    loop {
        match laser_triangulation_reading_receiver.try_changed() {
            Some(reading) => match reading {
                Safe(value) => {
                    defmt::info!("Range: {} mm (safe)", value)
                }
                Warning(value) => {
                    defmt::warn!("Range: {} mm (warning)", value)
                }
                Critical(value) => {
                    defmt::error!("Range: {} mm (critical)", value)
                }
            },
            None => (),
        }
    }
}
