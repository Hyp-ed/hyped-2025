use crate::io::Stm32f767ziGpioInput;
use embassy_stm32::gpio::Input;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_sensors::high_pressure::HighPressure;

#[embassy_executor::task]
pub async fn read_high_pressure(
    sp1_pin: Input<'static>,
    sp2_pin: Input<'static>,
    sender: Sender<'static, CriticalSectionRawMutex, Result<State, &'static str>, 1>,
) -> ! {
    let mut high_pressure_sensor = HighPressure::new(Stm32f767ziGpioInput::new(sp1_pin), Stm32f767ziGpioInput::new(sp2_pin));

    // update frequency of high pressure sensor in hz
    const UPDATE_FREQUENCY: u64 = 1000;

    loop {
        match high_pressure_sensor.get_high_pressure_state() {
            Ok(high_pressure_state) => Sender.send(high_pressure_state),
            Err(e) => Sender.send(e),
        }
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}