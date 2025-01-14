#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources};
use embassy_stm32::{
    bind_interrupts,
    eth::{self, generic_smi::GenericSMI, Ethernet, PacketQueue},
    peripherals::{self, ETH},
    rcc,
    rng::{self, Rng},
    time::Hertz,
    Config,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32h743zi::{
    config::{BOARD_STATIC_ADDRESS, GATEWAY_IP},
    log::log,
    tasks::{
        heartbeat::heartbeat,
        mqtt::{mqtt_recv_task, mqtt_send_task},
    },
};
use hyped_core::log_types::LogLevel;
use rand_core::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

/// Task for running the network stack
#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Bypass,
        });
        config.rcc.pll1 = Some(rcc::Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL216,
            divp: Some(rcc::PllDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // APB are additional clocks for external devices
        config.rcc.apb2_pre = APBPrescaler::DIV2; // and just need to be set to some value less
        config.rcc.apb3_pre = APBPrescaler::DIV2; // than the real clock
        config.rcc.apb4_pre = APBPrescaler::DIV2; //
        config.rcc.sys = Sysclk::PLL1_P; // 400Mhz
    }

    let p = embassy_stm32::init(config);

    let mut rng = Rng::new(p.RNG, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    let mac_addr: [u8; 6] = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];

    static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<4, 4>::new()),
        p.ETH,
        Irqs,
        p.PA1,
        p.PA2,
        p.PC1,
        p.PA7,
        p.PC4,
        p.PC5,
        p.PG13,
        p.PB13,
        p.PG11,
        GenericSMI::new(0),
        mac_addr,
    );

    let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: BOARD_STATIC_ADDRESS,
        dns_servers: heapless::Vec::new(),
        gateway: Some(GATEWAY_IP),
    });

    // Init network stack
    static STACK: StaticCell<Stack<Ethernet<'static, ETH, GenericSMI>>> = StaticCell::new();
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let stack = &*STACK.init(Stack::new(
        device,
        config,
        RESOURCES.init(StackResources::<3>::new()),
        seed,
    ));

    // Launch network task
    unwrap!(spawner.spawn(net_task(stack)));

    // Ensure DHCP configuration is up before trying connect
    stack.wait_config_up().await;

    log(LogLevel::Info, "Network stack initialized").await;

    // Launch MQTT send and receive tasks
    unwrap!(spawner.spawn(mqtt_send_task(stack)));
    unwrap!(spawner.spawn(mqtt_recv_task(stack)));
    unwrap!(spawner.spawn(heartbeat()));

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
