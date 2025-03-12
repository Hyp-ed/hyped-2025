use crate::io::Stm32f767ziGpioInput;
use embassy_stm32::gpio::Input;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_core::types::DigitalSignal;
use hyped_sensors::keyence::Keyence;

/// The update frequency of the Keyence sensor in Hz
const UPDATE_FREQUENCY: u64 = 10;

/// Test task that just continually updates the stripe count from the Keyence sensor (or other GPIO pin input)
#[embassy_executor::task]
pub async fn read_keyence(
    gpio_pin: Input<'static>,
    sender: Sender<'static, CriticalSectionRawMutex, u32, 1>,
) -> ! {
    let mut keyence = Keyence::new(Stm32f767ziGpioInput::new(gpio_pin), DigitalSignal::High);

    keyence.update_stripe_count();
    sender.send(keyence.get_stripe_count());

    loop {
        keyence.update_stripe_count();
        let new_stripe_count = Some(keyence.get_stripe_count());
        sender.send_if_modified(|old_stripe_count| {
            if new_stripe_count != *old_stripe_count {
                *old_stripe_count = new_stripe_count;
                true
            } else {
                false
            }
        });
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
