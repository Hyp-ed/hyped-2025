#![no_std]
#![no_main]

use core::str::FromStr;

use defmt::*;
use rand_core::RngCore;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_net::{tcp::TcpSocket, Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_stm32::{
    bind_interrupts,
    eth::{self, generic_smi::GenericSMI, Ethernet, PacketQueue},
    gpio::{AnyPin, Input, Pin, Pull},
    peripherals::{self, ETH},
    rng::{self, Rng},
    time::Hertz,
    Config,
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

// MQTT related imports
use heapless::String;
use rust_mqtt::{
    client::{client::MqttClient, client_config::ClientConfig},
    utils::rng_generator::CountingRng,
};
use typenum::consts::*;

use hyped_core::{
    format,
    format_string::show,
    log_types::LogLevel,
    mqtt::{initialise_mqtt_config, ButtonMqttMessage, HypedMqttClient, MqttMessage},
    mqtt_topics::MqttTopics,
};

bind_interrupts!(struct Irqs {
    ETH => eth::InterruptHandler;
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

static SEND_CHANNEL: Channel<ThreadModeRawMutex, MqttMessage, 128> = Channel::new();

async fn log(level: LogLevel, message: &str) {
    match level {
        LogLevel::Info => info!("{}", message),
        LogLevel::Warn => warn!("{}", message),
        LogLevel::Error => error!("{}", message),
        LogLevel::Debug => debug!("{}", message),
    }
    SEND_CHANNEL
        .send(MqttMessage {
            topic: MqttTopics::to_string(&MqttTopics::Logs),
            payload: String::<512>::from_str(message).unwrap(),
        })
        .await;
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) -> ! {
    stack.run().await
}

#[embassy_executor::task]
async fn button_task(pin: AnyPin) {
    let button: Input<_> = Input::new(pin, Pull::Down);
    loop {
        SEND_CHANNEL
            .send(MqttMessage {
                topic: MqttTopics::to_string(&MqttTopics::Acceleration),
                payload: String::<512>::from_str(
                    serde_json_core::to_string::<U512, ButtonMqttMessage>(&ButtonMqttMessage {
                        task_id: 0,
                        status: button.is_high(),
                    })
                    .unwrap()
                    .as_str(),
                )
                .unwrap(),
            })
            .await;
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn five_seconds_task() {
    loop {
        log(LogLevel::Info, "Ping from five second loop").await;
        SEND_CHANNEL
            .send(MqttMessage {
                topic: MqttTopics::to_string(&MqttTopics::Acceleration),
                payload: String::<512>::from_str(
                    serde_json_core::to_string::<U512, ButtonMqttMessage>(&ButtonMqttMessage {
                        task_id: 2,
                        status: false,
                    })
                    .unwrap()
                    .as_str(),
                )
                .unwrap(),
            })
            .await;
        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn mqtt_send_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    let mut rx_buffer: [u8; 4096] = [0; 4096];
    let mut tx_buffer: [u8; 4096] = [0; 4096];
    let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(60)));
    log(LogLevel::Info, "Connecting to Send Socket...").await;
    match socket
        .connect((Ipv4Address::new(169, 254, 195, 141), 1883))
        .await
    {
        Ok(()) => log(LogLevel::Info, "Connected to Send!").await,
        Err(connection_error) => {
            log(
                LogLevel::Error,
                format!(&mut [0u8; 1024], "Error connecting: {:?}", connection_error).unwrap(),
            )
            .await;
        }
    };

    let config = initialise_mqtt_config();
    let mut recv_buffer = [0; 1024];
    let mut write_buffer = [0; 1024];
    let client = MqttClient::<_, 5, _>::new(
        socket,
        &mut write_buffer,
        1024,
        &mut recv_buffer,
        1024,
        config,
    );
    let mut mqtt_client = HypedMqttClient { client };

    mqtt_client.connect_to_broker().await;

    loop {
        while !SEND_CHANNEL.is_empty() {
            let message = SEND_CHANNEL.receive().await;

            mqtt_client
                .send_message(message.topic.as_str(), message.payload.as_bytes(), false)
                .await;
        }
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn mqtt_recv_task(stack: &'static Stack<Ethernet<'static, ETH, GenericSMI>>) {
    let mut rx_buffer: [u8; 4096] = [0; 4096];
    let mut tx_buffer: [u8; 4096] = [0; 4096];
    let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(embassy_time::Duration::from_secs(600)));
    log(LogLevel::Info, "Connecting to Receive Socket...").await;
    match socket
        .connect((Ipv4Address::new(169, 254, 195, 141), 1883))
        .await
    {
        Ok(()) => {
            log(LogLevel::Info, "Connected to Receive!").await;
        }
        Err(connection_error) => {
            log(
                LogLevel::Error,
                format!(&mut [0u8; 1024], "Error connecting: {:?}", connection_error).unwrap(),
            )
            .await;
        }
    };
    let mut config = ClientConfig::new(
        rust_mqtt::client::client_config::MqttVersion::MQTTv5,
        CountingRng(10000),
    );
    config.add_max_subscribe_qos(rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1);
    config.max_packet_size = 100;
    config.add_client_id("receiver-stm-client");
    let mut recv_buffer = [0; 1024];
    let mut write_buffer = [0; 1024];
    let client = MqttClient::<_, 5, _>::new(
        socket,
        &mut write_buffer,
        1024,
        &mut recv_buffer,
        1024,
        config,
    );
    let mut mqtt_client = HypedMqttClient { client };
    mqtt_client.connect_to_broker().await;

    mqtt_client.subscribe("command_sender").await;
    mqtt_client.subscribe("acceleration").await;

    loop {
        match mqtt_client.receive_message().await {
            Ok((topic, message)) => {
                log(
                    LogLevel::Info,
                    format!(
                        &mut [0u8; 1024],
                        "Received message on topic {}: {}", topic, message
                    )
                    .unwrap(),
                )
                .await
            }
            Err(err) => {
                if err == rust_mqtt::packet::v5::reason_codes::ReasonCode::NetworkError {
                    break;
                }
                log(
                    LogLevel::Error,
                    format!(&mut [0u8; 1024], "Error receiving message: {:?}", err).unwrap(),
                )
                .await
            }
        }
        Timer::after(Duration::from_millis(100)).await;
    }
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
    }
    let p = embassy_stm32::init(config);
    spawner.spawn(button_task(p.PC13.degrade())).unwrap();

    log(LogLevel::Info, "Hello World!").await;

    // let seed: u64 = 0xdeadbeef;
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

    // let config = embassy_net::Config::dhcpv4(Default::default());
    let config = embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Address::new(169, 254, 195, 61), 24),
        dns_servers: heapless::Vec::new(),
        gateway: Some(Ipv4Address::new(169, 254, 195, 141)),
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
    unwrap!(spawner.spawn(mqtt_send_task(stack)));
    unwrap!(spawner.spawn(mqtt_recv_task(stack)));
    unwrap!(spawner.spawn(five_seconds_task()));
    loop {
        SEND_CHANNEL
            .send(MqttMessage {
                topic: MqttTopics::to_string(&MqttTopics::Acceleration),
                payload: String::<512>::from_str(
                    serde_json_core::to_string::<U512, ButtonMqttMessage>(&ButtonMqttMessage {
                        task_id: 1,
                        status: false,
                    })
                    .unwrap()
                    .as_str(),
                )
                .unwrap(),
            })
            .await;
        Timer::after(Duration::from_millis(1000)).await;
    }
}
