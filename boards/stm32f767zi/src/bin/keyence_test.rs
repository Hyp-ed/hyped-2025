#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{Can, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler},
    gpio::{Input, Pull},
    peripherals::CAN1,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use hyped_boards_stm32f767zi::tasks::{
    can::{heartbeat::heartbeat_responder, receive::can_receiver, send::can_sender},
    sensors::read_keyence::read_keyence,
    state_machine::state_updater,
};
use hyped_communications::{boards::Board, measurements::MeasurementId};
use hyped_state_machine::states::State;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
static BOARD: Board = Board::KeyenceTester;
pub static EMERGENCY: Watch<CriticalSectionRawMutex, bool, 1> = Watch::new();

/// Used to keep the latest stripe count from the Keyence sensor.
pub static CURRENT_KEYENCE_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let gpio_pin = Input::new(p.PC13, Pull::Down);

    let (can_tx, can_rx) = Can::new(p.CAN1, p.PD0, p.PD1, Irqs).split();
    spawner.must_spawn(can_receiver(can_rx, EMERGENCY.sender()));
    spawner.must_spawn(can_sender(can_tx));

    // Create a sender to pass to the temperature reading task, and a receiver for reading the values back.
    let mut receiver = CURRENT_KEYENCE_STRIPE_COUNT.receiver().unwrap();

    spawner.must_spawn(read_keyence(
        gpio_pin,
        BOARD,
        MeasurementId::Keyence1,
        CURRENT_KEYENCE_STRIPE_COUNT.sender(),
    ));
    spawner.must_spawn(state_updater(CURRENT_STATE.sender()));
    spawner.must_spawn(heartbeat_responder(BOARD));

    loop {
        // Only prints when the stripe count changes.
        let new_stripe_count = receiver.changed().await;
        defmt::info!("New stripe count: {}", new_stripe_count)
    }
}
