use core::cell::RefCell;

use crate::io::{Stm32f767ziCanRx, Stm32f767ziCanTx};
use defmt::*;
use embassy_stm32::{
    bind_interrupts,
    can::{
        filter::Mask32, Can, CanRx, CanTx, Fifo, Rx0InterruptHandler, Rx1InterruptHandler,
        SceInterruptHandler, TxInterruptHandler,
    },
    peripherals::CAN1,
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_time::Timer;
use hyped_can::HypedCanRx;
use hyped_sensors::imd::Imd;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

#[embassy_executor::task]
pub async fn read_imd() {
    let p = embassy_stm32::init(Default::default());

    static CAN: StaticCell<Can<'static>> = StaticCell::new();
    let can = CAN.init(Can::new(p.CAN1, p.PB8, p.PB9, Irqs));
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    can.modify_config().set_bitrate(500_000);
    can.enable().await;
    println!("CAN enabled");

    let (mut tx, mut rx) = can.split();

    let can_tx_mutex: Mutex<NoopRawMutex, RefCell<&mut CanTx<'_>>> =
        Mutex::new(RefCell::new(&mut tx));

    let mut can_tx = Stm32f767ziCanTx::new(&can_tx_mutex);

    let can_rx_mutex: Mutex<NoopRawMutex, RefCell<&mut CanRx<'_>>> =
        Mutex::new(RefCell::new(&mut rx));

    let mut can_rx = Stm32f767ziCanRx::new(&can_rx_mutex);

    let mut imd = Imd::new(&mut can_tx);

    loop {
        match imd.update_values() {
            Err(_) => {
                println!("Failed to update values.");
            }
            Ok(_) => {
                println!("IMD values updated successfully.");
            }
        }
        // allow time for response from imd...
        Timer::after_secs(1).await;

        let rcv_env = can_rx.read_frame();
        match rcv_env {
            Ok(env) => {
                let frame = env.frame;
                imd.process_message(frame);
                println!(
                    "Isolation status: {:?}, Positive resistance: {:?}, Negative resistance {:?}",
                    imd.get_isolation_status(),
                    imd.get_resistance_positive(),
                    imd.get_resistance_negative()
                );
            }
            Err(_) => {
                println!("Failed to receive frame from IMD");
            }
        }
    }
}
