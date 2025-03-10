#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, CanTx, Fifo, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
    StandardId, TxInterruptHandler,
};
use embassy_stm32::peripherals::CAN1;
use embassy_time::Timer;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

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

    let (tx, _rx) = can.split();

    static CAN_TX: StaticCell<CanTx<'static>> = StaticCell::new();
    let tx = CAN_TX.init(tx);

    let mut counter = 0;
    loop {
        println!("Counter: {}", counter);
        let frame = Frame::new_data(unwrap!(StandardId::new(0 as _)), &[counter]).unwrap();
        tx.write(&frame).await;
        Timer::after_secs(1).await;
        counter += 1;
    }
}
