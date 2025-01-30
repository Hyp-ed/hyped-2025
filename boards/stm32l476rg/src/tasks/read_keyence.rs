use crate::io::Stm32l476rgGpioInput;
use embassy_stm32::gpio::Input;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_sensors::keyence::Keyence;

/// Test task that just continually updates the stripe count from the Keyence sensor (or other GPIO pin input)
#[embassy_executor::task]
pub async fn read_keyence(
    gpio_pin: Input<'static>,
    sender: Sender<'static, CriticalSectionRawMutex, u32, 1>,
) -> ! {
    let mut keyence = Keyence::new(Stm32l476rgGpioInput::new(gpio_pin));

    loop {
        keyence.update_stripe_count();
        sender.send(keyence.get_stripe_count());
        Timer::after(Duration::from_millis(100)).await;
    }
}
