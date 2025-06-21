use crate::io::Stm32f767ziAdc;
use defmt_rtt as _;
use embassy_stm32::{
    adc::{Adc, AnyAdcChannel},
    peripherals::ADC1,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_core::config::SENSORS_CONFIG;
use hyped_sensors::{low_pressure::LowPressure, SensorValueRange};

/// Test task that just reads the pressure from the low pressure sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_low_pressure(
    adc: Adc<'static, ADC1>,
    pin: AnyAdcChannel<ADC1>,
    sender: Sender<'static, CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1>,
) -> ! {
    let hyped_adc = Stm32f767ziAdc::new(adc, pin, SENSORS_CONFIG.sensors.low_pressure.v_ref as f32);
    let mut low_pressure_sensor = LowPressure::new(hyped_adc);

    loop {
        sender.send(low_pressure_sensor.read_pressure());
        Timer::after(Duration::from_hz(
            SENSORS_CONFIG.sensors.low_pressure.update_frequency as u64,
        ))
        .await;
    }
}
