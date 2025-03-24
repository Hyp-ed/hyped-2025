#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::State;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, Fifo, Id, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
    TxInterruptHandler,
};
use embassy_stm32::peripherals::CAN1;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Watch;
use embassy_time::{Duration, Timer};
use hyped_can::HypedCanFrame;
use hyped_communications::measurements::MeasurementId;
use hyped_communications::messages::CanMessage;
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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    static CAN: StaticCell<Can<'static>> = StaticCell::new();
    let can = CAN.init(Can::new(p.CAN1, p.PD0, p.PD1, Irqs));
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    can.modify_config().set_bitrate(500_000);
    can.enable().await;
    println!("CAN enabled");

    let (_tx, mut rx) = can.split();

    loop {
        if let Ok(envelope) = rx.read().await {
            let id = envelope.frame.id();
            let can_id = match id {
                Id::Standard(id) => id.as_raw() as u32, // 11-bit ID
                Id::Extended(id) => id.as_raw(),        // 29-bit ID
            };

            let mut data = [0; 8];
            data.copy_from_slice(envelope.frame.data());
            let can_frame = HypedCanFrame::new(can_id, data);
            let can_message: CanMessage = can_frame.into();

            match can_message {
                CanMessage::MeasurementReading(measurement_reading) => {
                    let measurement_id = measurement_reading.measurement_id;

                    defmt::info!(
                        "Received measurement reading over CAN: {:?}",
                        measurement_id
                    );
                }
                CanMessage::StateTransitionCommand(state_transition) => {
                    defmt::info!(
                        "Received state transition over CAN: {:?}",
                        state_transition.to_state
                    );
                }
                CanMessage::StateTransitionRequest(state_transition_request) => {
                    defmt::info!(
                        "Received state transition request over CAN: {:?}",
                        state_transition_request.to_state
                    );
                }
                CanMessage::Heartbeat(heartbeat) => {
                    defmt::info!("Received heartbeat over CAN: {:?}", heartbeat.from);
                }
            }

            Timer::after(Duration::from_millis(100)).await
        }
    }
}
