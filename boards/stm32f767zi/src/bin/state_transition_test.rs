#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{Can, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler},
    peripherals::CAN1,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::{
    emergency, request_transition,
    tasks::{
        can::{can, send::CAN_SEND},
        state_machine::state_updater::state_updater,
    },
};
use hyped_communications::{
    boards::Board, messages::CanMessage, state_transition::StateTransition,
};
use hyped_state_machine::states::State;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

/// The current state of the state machine.
pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
pub static BOARD: Board = Board::StateMachineTester;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    spawner.must_spawn(can(Can::new(p.CAN1, p.PD0, p.PD1, Irqs)));
    spawner.must_spawn(state_updater(CURRENT_STATE.sender()));

    loop {
        Timer::after(Duration::from_secs(1)).await;
        request_transition!(State::Calibrate);
        Timer::after(Duration::from_secs(1)).await;
        request_transition!(State::Precharge);
        Timer::after(Duration::from_secs(1)).await;
        request_transition!(State::Accelerate);
        Timer::after(Duration::from_secs(1)).await;
        emergency!();
        Timer::after(Duration::from_secs(10)).await;
        panic!("End of test");
    }
}
