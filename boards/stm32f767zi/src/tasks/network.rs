use embassy_net::Stack;
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};

/// Task for running the network stack
#[embassy_executor::task]
pub async fn net_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) -> ! {
    stack.run().await
}

#[macro_export]
macro_rules! configure_networking {
    ($config:ident) => {{
        use embassy_stm32::rcc::*;
        $config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000),
            mode: HseMode::Bypass,
        });
        $config.rcc.pll_src = PllSource::HSE;
        $config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL216,
            divp: Some(PllPDiv::DIV2), // 8mhz / 4 * 216 / 2 = 216Mhz
            divq: None,
            divr: None,
        });
        $config.rcc.ahb_pre = AHBPrescaler::DIV1;
        $config.rcc.apb1_pre = APBPrescaler::DIV4;
        $config.rcc.apb2_pre = APBPrescaler::DIV2;
        $config.rcc.sys = Sysclk::PLL1_P;
    }};
}

#[macro_export]
macro_rules! set_up_network_stack {
    ($p:ident, $stack:ident, $spawner:ident) => {
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

        let config = embassy_net::Config::ipv4_static(StaticConfigV4 {
            address: Ipv4Cidr::new(
                Ipv4Address::from_str(TELEMETRY_CONFIG.networking.board.ip)
                    .expect("Invalid board IP address"),
                TELEMETRY_CONFIG.networking.board.subnet_mask as u8,
            ),
            dns_servers: heapless::Vec::new(),
            gateway: Some(
                Ipv4Address::from_str(TELEMETRY_CONFIG.networking.gateway.ip)
                    .expect("Invalid gateway IP address"),
            ),
        });

        static STACK: StaticCell<Stack<Ethernet<'static, ETH, GenericSMI>>> = StaticCell::new();
        static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
        let $stack = &*STACK.init(Stack::new(
            device,
            config,
            RESOURCES.init(StackResources::<3>::new()),
            seed,
        ));

        $spawner.spawn(net_task($stack)).unwrap();
        $stack.wait_config_up().await;

        log(LogLevel::Info, "Network stack initialized").await;
    };
}
