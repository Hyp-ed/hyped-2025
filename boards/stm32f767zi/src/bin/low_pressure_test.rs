#![no_std]
#![no_main]

use core::panic::PanicInfo;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::io::Stm32f767ziAdc;
use hyped_sensors::low_pressure::LowPressure;
use hyped_sensors::SensorValueRange::{self, *};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/// Used to keep the latest low pressure sensor value.
static LOW_PRESSURE_READING: Watch<CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1> =
    Watch::new();

/// Test task that just continually reads pressure from low pressure sensor and prints value to console
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    let adc = Adc::new(p.ADC1);
    let pin = p.PA3;

    // Create a sender to pass to the temperature reading task, and a receiver for reading the values back.
    // let low_pres_reading_sender = LOW_PRESSURE_READING_READING.sender();
    // let mut low_pres_reading_receiver = LOW_PRESSURE_READING_READING.receiver().unwrap();

    let mut low_pressure_sensor = LowPressure::new(Stm32f767ziAdc::new(adc, pin.degrade_adc()));

    let low_pres_reading = low_pressure_sensor.read_pressure();

    // spawner
    //     .low_pressure_sensor.read_pressure()

    // I DON'T UNDERSTAND
    loop {
        match low_pres_reading.try_changed() {
            Some(reading) => match reading {
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
            },
            None => (),
        }

        info!("{}", );
        Timer::after(Duration::from_millis(100)).await;
    }
}
