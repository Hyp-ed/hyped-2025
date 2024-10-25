use crate::io::gpio::Stm32l476rgGpio;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};
use hyped_sensors::keyence::Keyence;

/// Test task that just continually updates the stripe count from the Keyence sensor (or other GPIO pin input)
#[embassy_executor::task]
pub async fn read_keyence() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut keyence = Keyence::new(Stm32l476rgGpio::new(Input::new(p.PC13, Pull::Down)));

    loop {
        keyence.update_stripe_count();
        defmt::info!("Stripe count: {}", keyence.get_stripe_count());
        Timer::after(Duration::from_millis(100)).await;
    }
}
