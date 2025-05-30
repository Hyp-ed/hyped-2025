#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{Can, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler},
    gpio::{Input, Pull},
    peripherals::CAN1,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use hyped_boards_stm32f767zi::{
    board_state::THIS_BOARD,
    tasks::{
        can::{
            board_heartbeat::{heartbeat_listener, send_heartbeat},
            receive::can_receiver,
            send::can_sender,
        },
        sensors::read_keyence::read_keyence,
        state_machine::state_updater,
    },
};
use hyped_communications::boards::Board;
use hyped_core::config::MeasurementId;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

/// Used to keep the latest stripe count from the Keyence sensor.
pub static CURRENT_KEYENCE_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    THIS_BOARD
        .init(Board::KeyenceTester)
        .expect("Failed to initialize board");

    let p = embassy_stm32::init(Default::default());
    let gpio_pin = Input::new(p.PC13, Pull::Down);

    let (can_tx, can_rx) = Can::new(p.CAN1, p.PD0, p.PD1, Irqs).split();
    spawner.must_spawn(can_receiver(can_rx));
    spawner.must_spawn(can_sender(can_tx));

    // Create a sender to pass to the temperature reading task, and a receiver for reading the values back.
    let mut receiver = CURRENT_KEYENCE_STRIPE_COUNT.receiver().unwrap();

    spawner.must_spawn(read_keyence(
        gpio_pin,
        MeasurementId::Keyence1,
        CURRENT_KEYENCE_STRIPE_COUNT.sender(),
    ));
    spawner.must_spawn(state_updater());
    spawner.must_spawn(heartbeat_listener(Board::Telemetry));
    spawner.must_spawn(send_heartbeat(Board::Telemetry));

    loop {
        // Only prints when the stripe count changes.
        let new_stripe_count = receiver.changed().await;
        defmt::info!("New stripe count: {}", new_stripe_count)
    }
}
