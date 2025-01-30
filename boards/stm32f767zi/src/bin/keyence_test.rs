#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use hyped_boards_stm32f767zi::tasks::read_keyence::read_keyence;
use {defmt_rtt as _, panic_probe as _};

/// Used to keep the latest temperature sensor value.
static KEYENCE_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let gpio_pin = Input::new(p.PC13, Pull::Down);

    // Create a sender to pass to the temperature reading task, and a receiver for reading the values back.
    let sender = KEYENCE_STRIPE_COUNT.sender();
    let mut receiver = KEYENCE_STRIPE_COUNT.receiver().unwrap();

    spawner.spawn(read_keyence(gpio_pin, sender)).unwrap();

    // Only prints when the stripe count changes.
    loop {
        let new_stripe_count = receiver.get().await;
        defmt::info!("New stripe count: {}", new_stripe_count)
    }
}
