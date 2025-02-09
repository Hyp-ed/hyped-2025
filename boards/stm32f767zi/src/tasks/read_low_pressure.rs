use crate::io::Stm32f767ziAdc;
use embassy_time::{Duration, Timer};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use hyped_sensors::low_pressure::LowPressure;

/// Test task that just continually reads pressure from low pressure sensor and prints value to console
#[embassy_executor::task]
pub async fn read_low_pressure(
    adc_pin: Adc<'d, T>,
    adc_channel: AnyAdcChannel<T>,
    sender: Sender<'static, CriticalSectionRawMutex, u32, 1>,
) -> ! {
    let mut low_pressure_sensor = LowPressure::new(Stm32f767ziAdc::new(adc_pin, adc_channel));

    loop {
        sender.send(low_pressure_sensor.read_pressure());
        Timer::after(Duration::from_millis(100)).await;
    }
}

// replace "T" type later when I understand what is needed
