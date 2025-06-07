#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::sdmmc::Sdmmc;
use embassy_stm32::time::{mhz, Hertz};
use embassy_stm32::{bind_interrupts, peripherals, sdmmc, Config};
use {defmt_rtt as _, panic_probe as _};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;

bind_interrupts!(struct Irqs {
    SDMMC1 => sdmmc::InterruptHandler<peripherals::SDMMC1>;
});

// Adapted from https://github.com/embassy-rs/embassy/blob/bcebe4c4d5b597da0b8741916e450c46e6fef06e/examples/stm32f7/src/bin/sdmmc.rs

static CHANNEL: Channel<ThreadModeRawMutex, &'static str, 4> = Channel::new();

async fn process_queue(sdmmc: &mut Sdmmc<'static, peripherals::SDMMC1>) {
    let mut buffer = [0u8; 512];

    loop {
        let message = CHANNEL.recv().await;
        let bytes = message.as_bytes();
        buffer[..bytes.len()].copy_from_slice(bytes);

        write_to_sdmmc(sdmmc, &buffer).await;
    }
}

fn create_config() -> Config {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Bypass,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL216,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 216 / 2 = 216Mhz
            divq: Some(PllQDiv::DIV9), // 8mhz / 4 * 216 / 9 = 48Mhz
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }
    config
}

async fn initialize_sdmmc(p: &embassy_stm32::Peripherals) -> Sdmmc<'static, peripherals::SDMMC1> {
    let mut sdmmc = Sdmmc::new_4bit(
        p.SDMMC1,
        Irqs,
        p.DMA2_CH3,
        p.PC12,
        p.PD2,
        p.PC8,
        p.PC9,
        p.PC10,
        p.PC11,
        Default::default(),
    );

    unwrap!(sdmmc.init_card(mhz(25)).await);
    sdmmc
}

async fn read_from_sdmmc(sdmmc: &mut Sdmmc<'static, peripherals::SDMMC1>, buffer: &mut [u8; 512]) {
    unwrap!(sdmmc.read_block(0, buffer).await);
}

async fn write_to_sdmmc(sdmmc: &mut Sdmmc<'static, peripherals::SDMMC1>, buffer: &[u8; 512]) {
    let mut buffer = *buffer;
    buffer[0] = 0xAA;
    unwrap!(sdmmc.write_block(0, &buffer).await);
}

async fn close_sdmmc(sdmmc: &mut Sdmmc<'static, peripherals::SDMMC1>) {
    unwrap!(sdmmc.deinit_card().await);
}