#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources};
use embassy_stm32::{
    bind_interrupts,
    can::{Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler},
    eth::{self, generic_smi::GenericSMI, Ethernet, PacketQueue},
    peripherals::{self, CAN1, ETH},
    rng::{self, Rng},
    time::Hertz,
    Config,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::{
    log::log,
    tasks::{
        can::{can, heartbeat_controller::heartbeat_controller},
        mqtt::heartbeat::base_station_heartbeat,
        network::net_task,
        state_machine::state_machine::state_machine,
        tasks::mqtt::mqtt,
    },
    telemetry_config::{BOARD_STATIC_ADDRESS, GATEWAY_IP},
};
use hyped_communications::boards::Board;
use hyped_core::log_types::LogLevel;
use rand_core::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

const BOARD: Board = Board::Telemetry;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    configure_networking!(config);
    let p = embassy_stm32::init(config);
    set_up_network_stack!(p, config, stack, spawner);

    // Network tasks: MQTT and base station heartbeat
    spawner.must_spawn(mqtt(stack));
    spawner.must_spawn(base_station_heartbeat());

    // CAN tasks: CAN send/receive, heartbeat controller, and state machine
    spawner.must_spawn(can(Can::new(p.CAN1, p.PD0, p.PD1, Irqs)));
    // Spawn a task for each board we want to keep track of
    spawner.must_spawn(heartbeat_controller(BOARD, Board::Navigation));
    // ... add more boards here
    spawner.must_spawn(state_machine(BOARD, state_sender));

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[macro_export]
macro_rules! configure_networking {
    ($config:ident) => {{
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
            divq: None,
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }};
}

#[macro_export]
macro_rules! set_up_network_stack {
    ($p:ident, $config:ident, $stack:ident, $spawner:ident) => {
        let mut rng = Rng::new($p.RNG, Irqs);
        let mut seed = [0; 8];
        rng.fill_bytes(&mut seed);
        let seed = u64::from_le_bytes(seed);

        let mac_addr: [u8; 6] = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];

        static PACKETS: StaticCell<PacketQueue<4, 4>> = StaticCell::new();
        let device = Ethernet::new(
            PACKETS.init(PacketQueue::<4, 4>::new()),
            $p.ETH,
            Irqs,
            $p.PA1,
            $p.PA2,
            $p.PC1,
            $p.PA7,
            $p.PC4,
            $p.PC5,
            $p.PG13,
            $p.PB13,
            $p.PG11,
            GenericSMI::new(0),
            mac_addr,
        );

        static STACK: StaticCell<Stack<Ethernet<'static, ETH, GenericSMI>>> = StaticCell::new();
        static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
        let stack = &*STACK.init(Stack::new(
            device,
            $config,
            RESOURCES.init(StackResources::<3>::new()),
            seed,
        ));

        $spawner.spawn(net_task(stack)).unwrap();

        stack.wait_config_up().await;

        log(LogLevel::Info, "Network stack initialized").await;
    };
}
