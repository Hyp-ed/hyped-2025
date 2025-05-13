#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::tasks::sensors::read_low_pressure::read_low_pressure;
use hyped_sensors::SensorValueRange::{self, *};
use {defmt_rtt as _, panic_probe as _};

/// The update frequency of the low pressure sensor
const UPDATE_FREQUENCY: Duration = Duration::from_hz(10);
/// Used to keep the latest low pressure sensor value.
static LOW_PRESSURE_READING: Watch<CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    // Create a sender to pass to the low pressure reading task, and a receiver for reading the values back.
    let low_pressure_reading_sender = LOW_PRESSURE_READING.sender();
    let mut low_pressure_reading_receiver = LOW_PRESSURE_READING.receiver().unwrap();

    // Initialize the ADC peripheral and the pin that the low pressure sensor is connected to.
    let adc = Adc::new(p.ADC1);
    let pin = p.PA3.degrade_adc();

    spawner.must_spawn(read_low_pressure(adc, pin, low_pressure_reading_sender));

    // Every `UPDATE_FREQUENCY` we read for the latest value from the low pressure sensor.
    loop {
        if let Some(reading) = low_pressure_reading_receiver.try_changed() {
            match reading {
                Some(reading) => match reading {
                    Safe(low_pres) => {
                        defmt::info!("Pressure: {} bar (safe)", low_pres);
                    }
                    Warning(low_pres) => {
                        defmt::warn!("Pressure: {} bar (warning)", low_pres);
                    }
                    Critical(low_pres) => {
                        defmt::error!("Pressure: {} bar (critical)", low_pres);
                    }
                },
                None => defmt::warn!("No low pressure reading available"),
            }
        }
        Timer::after(UPDATE_FREQUENCY).await;
    }
}
