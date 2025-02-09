#![no_std]
#![no_main]

use crate::io::Stm32f767ziAdc;
use embassy_stm32::adc::{Adc, AdcChannel, AnyAdcChannel};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_sensors::low_pressure::LowPressure;

/// Test task that just continually reads pressure from low pressure sensor and prints value to console
#[embassy_executor::main]
pub async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    let mut adc = Adc::new(p.ADC1);
    let pin = p.PA3;

    let sender: Sender<'static, CriticalSectionRawMutex, f32, 1>;

    let mut low_pressure_sensor = LowPressure::new(Stm32f767ziAdc::new(adc, pin.degrade_adc()));

    loop {
        sender.send(low_pressure_sensor.read_pressure());
        Timer::after(Duration::from_millis(100)).await;
    }
}
