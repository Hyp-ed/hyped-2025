#![no_std]
#![no_main]

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
use embassy_time::Timer;
use hyped_adc::HypedAdc;
use hyped_boards_stm32f767zi::io::Stm32f767ziAdc;
use hyped_sensors::low_pressure::LowPressure;
use hyped_sensors::SensorValueRange::{self, *};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

type Adc1Bus = Mutex<NoopRawMutex, RefCell<Adc<'static>>>;

/// Used to keep the latest low pressure sensor value.
static LOW_PRESSURE_READING: Watch<CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut adc = Adc::new(p.ADC1);

    // Initialize the ADC and store it in a static cell so that it can be accessed from the tasks.
    static ADC: StaticCell<I2c1Bus> = StaticCell::new();
    let adc_access = ADC.init(Mutex::new(RefCell::new(adc)));

    // Create a sender to pass to the low pressure reading task, and a receiver for reading the values back.
    let low_pressure_reading_sender = LOW_PRESSURE_READING.sender();
    let mut low_pressure_reading_receiver = LOW_PRESSURE_READING.receiver().unwrap();

    spawner
        .spawn(read_low_pressure(adc_access, low_pressure_reading_sender))
        .unwrap();

    // Every 100ms we read for the latest value from the low pressure sensor.
    loop {
        match low_pressure_reading_receiver.try_changed() {
            Some(reading) => match reading {
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
                None => defmt::warn!("No low pressure reading available."),
            },
            None => (),
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}