use crate::io::Stm32f767ziAdc;
use embassy_stm32::adc::{Adc, AnyAdcChannel};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_sensors::low_pressure::LowPressure;

/// Test that just continually reads pressure from low pressure sensor and prints value to console
#[test]
pub async fn read_low_pressure<T>(
    adc_pin: Adc<'static, T>,
    adc_channel: AnyAdcChannel<T>,
    sender: Sender<'static, CriticalSectionRawMutex, f32, 1>,
) -> ! {
    let mut low_pressure_sensor = LowPressure::new(Stm32f767ziAdc::new(adc_pin, adc_channel));

    loop {
        sender.send(low_pressure_sensor.read_pressure());
        Timer::after(Duration::from_millis(100)).await;
    }
}
