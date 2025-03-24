#![no_std]
#![no_main]

use core::panic;

use embassy_executor::Spawner;
use embassy_net::{Stack, StackResources};
use embassy_stm32::{
    bind_interrupts,
    can::{Can, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler},
    eth::{self, generic_smi::GenericSMI, Ethernet, PacketQueue},
    peripherals::{self, CAN1, ETH},
    rng::{self, Rng},
    time::Hertz,
    Config,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use hyped_boards_stm32f767zi::{
    configure_networking,
    log::log,
    set_up_network_stack,
    tasks::{
        can::{heartbeat::heartbeat_controller, receive::can_receiver, send::can_sender},
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
    spawner.must_spawn(base_station_heartbeat());

    // CAN tasks: CAN send/receive, heartbeat controller, and state machine
    let (can_tx, can_rx) = Can::new(p.CAN1, p.PD0, p.PD1, Irqs).split();
    spawner.must_spawn(can_receiver(can_rx, EMERGENCY.sender()));
    spawner.must_spawn(can_sender(can_tx));

    // Spawn a task for each board we want to keep track of
    spawner.must_spawn(heartbeat_controller(BOARD, Board::Navigation));
    // ... add more boards here
    spawner.must_spawn(state_machine(BOARD, CURRENT_STATE.sender()));

    let current_state_sender = CURRENT_STATE.sender();

    loop {
        // All main loops should have logic to handle an emergency signal...
        if EMERGENCY.receiver().unwrap().get().await {
            // ... and take appropriate action
            current_state_sender.send(State::EmergencyBrake);
            panic!("Emergency signal received");
        }
    }
}
