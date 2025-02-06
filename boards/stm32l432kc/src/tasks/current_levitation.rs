use embassy_stm32::adc::Adc;
use embassy_time::Delay;
use hyped_sensors::{current_levitation::CurrentLevitation, SensorValueRange::*};
use defmt_rtt as _;


/// Test task that reads the current and prints it to console
#[embassy_executor::task]
pub async fn read_current_levitation() -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1, Delay);

    let mut current_levitation_sensor = CurrentLevitation::new(&mut adc);

    loop {
        match current_levitation_sensor.read() {
            Safe(value) => {
                defmt::info!("Current: {} A (safe)", value)
            }
            Warning(value) => {
                defmt::warn!("Current: {} A (warning)", value)
            }
            Critical(value) => {
                defmt::error!("Current: {} A (critical)", value)
            }
        }
    }
}
