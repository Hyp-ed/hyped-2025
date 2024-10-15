#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l476rg::tasks::keyence::read_keyence;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    spawner.spawn(read_keyence()).unwrap();

    // Some other tasks for navigation here...

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
