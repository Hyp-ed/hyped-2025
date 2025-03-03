#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{
        filter::Mask32, Can, Fifo, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
        TxInterruptHandler,
    },
    peripherals::CAN1,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::{
    emergency, request_transition,
    tasks::{
        can_receiver::can_receiver,
        can_sender::{can_sender, CAN_SEND},
        state_updater::state_updater,
    },
};
use hyped_core::{
    comms::{boards::Board, messages::CanMessage, state_transition::StateTransition},
    states::State,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

/// The current state of the state machine.
pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
static BOARD: Board = Board::StateMachineTester;

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
