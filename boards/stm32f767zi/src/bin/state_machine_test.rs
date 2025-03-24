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
use hyped_boards_stm32f767zi::tasks::{
    can::{heartbeat::heartbeat_controller, receive::can_receiver, send::can_sender},
    state_machine::state_machine,
};
use hyped_communications::boards::Board;
use hyped_state_machine::states::State;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
static BOARD: Board = Board::StateMachineTester;
pub static EMERGENCY: Watch<CriticalSectionRawMutex, bool, 1> = Watch::new();

pub static TARGET_BOARD: Board = Board::Test;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    let (can_tx, can_rx) = Can::new(p.CAN1, p.PD0, p.PD1, Irqs).split();
    spawner.must_spawn(can_receiver(can_rx, EMERGENCY.sender()));
    spawner.must_spawn(can_sender(can_tx));

    spawner.must_spawn(state_machine(
        Board::TemperatureTester,
        CURRENT_STATE.sender(),
    ));
    spawner.must_spawn(heartbeat_controller(BOARD, TARGET_BOARD));

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
