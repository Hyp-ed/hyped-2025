#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{
        filter::Mask32, Can, Fifo, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
        TxInterruptHandler,
    },
    gpio::Pin,
    peripherals::CAN1,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::tasks::{
    can_receiver::can_receiver, can_sender::can_sender,
    heartbeat_coordinator::heartbeat_coordinator, state_machine::state_machine,
};
use hyped_core::{comms::boards::Board, states::State};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/// The current state of the state machine.
pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
static _BOARD: Board = Board::StateMachineTester;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    // Initialise CAN
    static CAN: StaticCell<Can<'static>> = StaticCell::new();
    let can = CAN.init(Can::new(p.CAN1, p.PD0, p.PD1, Irqs));
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    can.modify_config().set_bitrate(500_000);
    can.enable().await;
    defmt::info!("CAN enabled");

    let (tx, rx) = can.split();

    spawner.must_spawn(can_receiver(rx));
    spawner.must_spawn(can_sender(tx));
    spawner.must_spawn(state_machine(
        Board::TemperatureTester,
        CURRENT_STATE.sender(),
    ));
    spawner.must_spawn(heartbeat_coordinator(_BOARD));

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
