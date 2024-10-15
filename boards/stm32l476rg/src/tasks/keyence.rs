use crate::io::gpio::Stm32l476rgGpio;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};
use hyped_sensors::keyence::Keyence;

#[embassy_executor::task]
pub async fn read_keyence() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut keyence = Keyence::new(Stm32l476rgGpio::new(Input::new(p.PC13, Pull::Down)));

    loop {
        keyence.update_stripe_count().unwrap();
        defmt::info!("Stripe count: {}", keyence.get_stripe_count());
        Timer::after(Duration::from_millis(100)).await;
    }
}
