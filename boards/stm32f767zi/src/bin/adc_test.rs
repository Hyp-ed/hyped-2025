#![no_std]
#![no_main]

// "Liberated" from https://github.com/embassy-rs/embassy/blob/6c4b3d82b637fce5ab6efdc312d7852381d8ddeb/examples/stm32f7/src/bin/adc.rs
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_time::Timer;
use hyped_adc::HypedAdc;
use hyped_boards_stm32f767zi::io::Stm32f767ziAdc;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut adc = Adc::new(p.ADC1);
    let pin = p.PA3;

    let mut vrefint = adc.enable_vrefint();
    let vrefint_sample = adc.blocking_read(&mut vrefint);
    let convert_to_millivolts = |sample| {
        // From http://www.st.com/resource/en/datasheet/DM00273119.pdf
        // 6.3.27 Reference voltage
        const VREFINT_MV: u32 = 1210; // mV

        (u32::from(sample) * VREFINT_MV / u32::from(vrefint_sample)) as u16
    };

    let mut hyped_adc = Stm32f767ziAdc::new(adc, pin.degrade_adc());

    loop {
        let v = hyped_adc.read_value();
        info!("--> {} - {} mV", v, convert_to_millivolts(v));
        Timer::after_millis(100).await;
    }
}
