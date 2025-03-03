#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::{
    emergency, request_transition,
    tasks::{
        can::{can, CAN_SEND},
        state_updater::state_updater,
    },
};
use hyped_core::{
    comms::{boards::Board, messages::CanMessage, state_transition::StateTransition},
    states::State,
};
use {defmt_rtt as _, panic_probe as _};

/// The current state of the state machine.
pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
static BOARD: Board = Board::StateMachineTester;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    spawner.must_spawn(can(p.CAN1, p.PD0, p.PD1));

    spawner.must_spawn(state_updater(CURRENT_STATE.sender()));

    let can_sender = CAN_SEND.sender();

    loop {
        Timer::after(Duration::from_secs(1)).await;
        request_transition!(State::Calibrate, can_sender, BOARD);
        Timer::after(Duration::from_secs(1)).await;
        request_transition!(State::Precharge, can_sender, BOARD);
        Timer::after(Duration::from_secs(1)).await;
        request_transition!(State::Accelerate, can_sender, BOARD);
        Timer::after(Duration::from_secs(1)).await;
        emergency!(can_sender, BOARD);
        Timer::after(Duration::from_secs(10)).await;
        panic!("End of test");
    }
}
