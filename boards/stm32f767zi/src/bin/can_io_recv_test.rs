#![no_std]
#![no_main]

use core::cell::RefCell;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, Fifo, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler, TxInterruptHandler,
};
use embassy_stm32::peripherals::CAN1;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use hyped_boards_stm32f767zi::io::Stm32f767ziCan;
use hyped_can::HypedCan;
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
    let can = CAN.init(Can::new(p.CAN1, p.PB8, p.PB9, Irqs));
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    can.modify_config().set_bitrate(500_000);
    can.enable().await;
    println!("CAN enabled");

    let can_mutex: Mutex<NoopRawMutex, RefCell<&mut Can<'_>>> = Mutex::new(RefCell::new(can));

    let mut can_io = Stm32f767ziCan::new(&can_mutex);

    loop {
        let recv_frame = can_io.read_frame();
        match recv_frame {
            Ok(frame) => {
                println!(
                    "Received frame: ID: {:?}, Data: {:?}",
                    frame.frame.can_id, frame.frame.data
                );
            }
            Err(_) => continue,
        }
    }
}
