#![no_std]
#![no_main]

use core::panic;

use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources};
use embassy_stm32::{
    bind_interrupts,
    can::{
        filter::Mask32, Can, Fifo, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
        TxInterruptHandler,
    },
    eth::{self, generic_smi::GenericSMI, Ethernet, PacketQueue},
    peripherals::{self, CAN1, ETH},
    rng::{self, Rng},
    time::Hertz,
    Config,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::{
    board_state::{CURRENT_STATE, EMERGENCY, THIS_BOARD},
    configure_networking, default_can_config,
    log::log,
    set_up_network_stack,
    tasks::{
        can::{
            board_heartbeat::{heartbeat_listener, send_heartbeat},
            receive::can_receiver,
            send::can_sender,
        },
        can_to_mqtt::can_to_mqtt,
        mqtt::{base_station_heartbeat::base_station_heartbeat, mqtt},
        network::net_task,
        state_machine::state_machine,
    },
    telemetry_config::{BOARD_STATIC_ADDRESS, GATEWAY_IP},
};
use hyped_communications::boards::Board;
use hyped_core::log_types::LogLevel;
use hyped_state_machine::states::State;
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

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    THIS_BOARD
        .init(Board::Telemetry)
        .expect("Failed to initialize board");

    let mut config = Config::default();
    configure_networking!(config);
    let p = embassy_stm32::init(config);
    set_up_network_stack!(p, stack, spawner);

    // Network tasks: MQTT and base station heartbeat
    spawner.must_spawn(mqtt(stack));
    Timer::after(Duration::from_secs(2)).await;
    spawner.must_spawn(base_station_heartbeat());

    // CAN tasks: CAN send/receive, heartbeat controller, and state machine
    defmt::info!("Setting up CAN...");
    let mut can = Can::new(p.CAN1, p.PD0, p.PD1, Irqs);
    default_can_config!(can);
    can.enable().await;
    let (can_tx, can_rx) = can.split();
    spawner.must_spawn(can_receiver(can_rx));
    spawner.must_spawn(can_sender(can_tx));
    defmt::info!("CAN setup complete");

    spawner.must_spawn(can_to_mqtt());
    spawner.must_spawn(emergency_handler());
    spawner.must_spawn(heartbeat_listener(Board::TemperatureTester));
    spawner.must_spawn(send_heartbeat(Board::TemperatureTester));
    // ... add more boards here
    spawner.must_spawn(state_machine());

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn emergency_handler() {
    let current_state_sender = CURRENT_STATE.sender();

    loop {
        // All main loops should have logic to handle an emergency signal...
        if EMERGENCY.receiver().unwrap().get().await {
            defmt::error!("Emergency signal received! Cleaning up...");
            // ... and take appropriate action
            current_state_sender.send(State::Emergency);
            // Wait for the emergency signal to be sent
            Timer::after(Duration::from_secs(1)).await;
            panic!("Terminating due to emergency signal!");
        }
    }
}
