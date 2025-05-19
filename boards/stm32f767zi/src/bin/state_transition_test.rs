#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{Can, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler},
    peripherals::CAN1,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::{
    board_state::{CURRENT_STATE, EMERGENCY, THIS_BOARD},
    emergency, request_transition,
    tasks::{
        can::{
            receive::can_receiver,
            send::{can_sender, CAN_SEND},
        },
        state_machine::state_updater,
    },
};
use hyped_communications::{
    emergency::Reason, messages::CanMessage, state_transition::StateTransitionRequest,
};
use hyped_state_machine::states::State;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    let (can_tx, can_rx) = Can::new(p.CAN1, p.PD0, p.PD1, Irqs).split();
    spawner.must_spawn(can_receiver(can_rx));
    spawner.must_spawn(can_sender(can_tx));

    spawner.must_spawn(state_updater());

    Timer::after(Duration::from_secs(1)).await;
    request_transition!(State::Calibrate);
    Timer::after(Duration::from_secs(1)).await;
    request_transition!(State::Precharge);
    Timer::after(Duration::from_secs(1)).await;
    request_transition!(State::Accelerate);
    Timer::after(Duration::from_secs(1)).await;
    emergency!(Reason::Test);

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
