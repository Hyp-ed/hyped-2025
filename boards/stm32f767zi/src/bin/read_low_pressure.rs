use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_time::{Duration, Timer};
use hyped_adc::HypedAdc;
use hyped_boards_stm32f767zi::io::Stm32f767ziAdc;
use hyped_sensors::low_pressure::LowPressure;

/// Test task that just continually reads pressure from low pressure sensor and prints value to console
#[embassy_executor::main]
pub async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Low pressure sensor");

    let adc = Adc::new(p.ADC1);
    let pin = p.PA3;

    let mut low_pressure_sensor = LowPressure::new(Stm32f767ziAdc::new(adc, pin.degrade_adc()));

    loop {
        info!("{}", low_pressure_sensor.read_pressure());
        Timer::after(Duration::from_millis(100)).await;
    }
}
