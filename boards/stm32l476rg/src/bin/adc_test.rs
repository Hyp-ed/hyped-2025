#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{
    adc::{Adc, AdcChannel, Resolution},
    Config,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l476rg::io::adc::Stm32l476rgAdc;
use hyped_io::adc::HypedAdc;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    info!("Hello World!");

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.mux.adcsel = mux::Adcsel::SYS;
    }
    let p = embassy_stm32::init(config);

    let mut adc = Adc::new(p.ADC1);
    adc.set_resolution(Resolution::BITS12);
    let mut channel = p.PA0;

    loop {
        let v = adc.blocking_read(&mut channel);
        info!("--> {}", v);
    }
}
