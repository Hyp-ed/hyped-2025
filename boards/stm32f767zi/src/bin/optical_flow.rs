#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{self, BitOrder, Spi};
use embassy_stm32::time::khz;
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::io::{Stm32f767ziGpioOutput, Stm32f767ziSpi};
use hyped_sensors::optical_flow::OpticalFlow;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    let mut spi_config = spi::Config::default();
    spi_config.frequency = khz(400);
    spi_config.bit_order = BitOrder::MsbFirst;

    let spi = Spi::new_blocking(p.SPI1, p.PB3, p.PB5, p.PB4, spi_config);
    let mut hyped_spi = Stm32f767ziSpi::new(spi);

    let cs = Stm32f767ziGpioOutput::new(Output::new(p.PA4, Level::High, Speed::VeryHigh));

    let mut optical_flow = OpticalFlow::new(&mut hyped_spi, cs)
        .await
        .expect("Failed to initialize optical flow sensor.");

    defmt::info!("Optical flow sensor initialized.");

    loop {
        let flow = optical_flow.get_motion().await.unwrap();
        defmt::info!("Flow: {:?}", flow);
        Timer::after(Duration::from_millis(500)).await;
    }
}
