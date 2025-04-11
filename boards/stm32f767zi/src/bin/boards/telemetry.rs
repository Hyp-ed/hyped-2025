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
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::{
    configure_networking,
    log::log,
    set_up_network_stack,
    tasks::{
        can::{heartbeat::heartbeat_controller, receive::can_receiver, send::can_sender},
        can_to_mqtt::can_to_mqtt,
        mqtt::{heartbeat::base_station_heartbeat, mqtt},
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

// All boards should have these:
const BOARD: Board = Board::Telemetry;
pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
pub static EMERGENCY: Watch<CriticalSectionRawMutex, bool, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    configure_networking!(config);
    let p = embassy_stm32::init(config);
    set_up_network_stack!(p, stack, spawner);

    // Network tasks: MQTT and base station heartbeat
    spawner.must_spawn(mqtt(stack));
    Timer::after(Duration::from_secs(2)).await;
    // spawner.must_spawn(base_station_heartbeat());

    // CAN tasks: CAN send/receive, heartbeat controller, and state machine
    defmt::info!("Setting up CAN...");
    let mut can = Can::new(p.CAN1, p.PD0, p.PD1, Irqs);
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    can.modify_config().set_bitrate(500_000);
    can.enable().await;
    let (can_tx, can_rx) = can.split();
    spawner.must_spawn(can_receiver(can_rx, EMERGENCY.sender()));
    spawner.must_spawn(can_sender(can_tx));
    defmt::info!("CAN setup complete");

    spawner.must_spawn(can_to_mqtt());

    // Spawn a task for each board we want to keep track of
    // spawner.must_spawn(heartbeat_controller(BOARD, Board::TemperatureTester));
    // ... add more boards here
    spawner.must_spawn(state_machine(BOARD, CURRENT_STATE.sender()));

    let current_state_sender = CURRENT_STATE.sender();

    loop {
        // All main loops should have logic to handle an emergency signal...
        if EMERGENCY.receiver().unwrap().get().await {
            // ... and take appropriate action
            current_state_sender.send(State::Emergency);
            panic!("Emergency signal received");
        }
    }
}
