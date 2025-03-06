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
use {defmt_rtt as _, panic_probe as _};

type Adc1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static>>>;

/// Used to keep the latest low pressure sensor value.
static LOW_PRESSURE_READING: Watch<CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1> =
    Watch::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1);
    let pin = p.PA3;

    // tbc
}