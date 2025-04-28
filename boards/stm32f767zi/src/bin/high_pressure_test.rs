#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use hyped_boards_stm32f767zi::tasks::read_high_pressure::read_high_pressure;
use {defmt_rtt as _, panic_probe as _};

/// used to store latest high pressure sensor value
static HIGH_PRESSURE_SENSOR_VALUE: Watch<CriticalSectionRawMutex, Result<State, &'static str>, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let sp1_gpio = Input::new(p.PC12, Pull::Down);
    let sp2_gpio = Input::new(p.PC13, Pull::Down);

    
}