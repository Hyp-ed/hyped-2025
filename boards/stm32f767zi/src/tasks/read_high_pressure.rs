use crate::io::Stm32f767ziGpioInput;
use embassy_stm32::gpio::Input;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_sensors::high_pressure::HighPressure;

#[embassy_executor::task]
pub async fn read_high_pressure(
    sp1_pin: Input<'static>,
    sp2_pin: Input<'static>,
    sender: Sender<'static, CriticalSectionRawMutex, u32, 1>,
) -> ! {
    let mut high_pressure_sensor = HIghPressure::new(Stm32f767ziGpioInput::new(sp1_pin), Stm32f767ziGpioInput::new(sp2_pin));

    loop {
        sender.send();  // match get high pressure results
        Timer::after(Duration::from_millis(100)).await;
    }
}