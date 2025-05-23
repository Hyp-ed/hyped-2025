#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l476rg::tasks::time_of_flight::read_time_of_flight_range;
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    spawner.spawn(read_time_of_flight_range()).unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
