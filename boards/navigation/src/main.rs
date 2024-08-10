#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_time::{Duration, Timer};
use hyped_sensors::keyence;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut gpio_input = Input::new(p.PA0, Pull::Up);

    let mut keyence = keyence::Keyence::new(gpio_input);

    loop {
        keyence.update_stripe_count().unwrap();
        defmt::info!("Stripe count: {}", keyence.get_stripe_count());
        Timer::after(Duration::from_millis(100)).await;
    }
}
