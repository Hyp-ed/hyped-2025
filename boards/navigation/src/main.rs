#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};
use hyped_io::gpio::EmbassyGpio;
use hyped_sensors::keyence::Keyence;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let gpio_input = Input::new(p.PA0, Pull::Up);
    let gpio = EmbassyGpio::new(gpio_input);
    let mut keyence = Keyence::new(gpio);

    loop {
        keyence.update_stripe_count().unwrap();
        defmt::info!("Stripe count: {}", keyence.get_stripe_count());
        Timer::after(Duration::from_millis(100)).await;
    }
}
