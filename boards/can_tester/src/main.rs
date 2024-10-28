// #![no_std]
// #![no_main]

// use core::num::{NonZeroU16, NonZeroU8};

// use defmt::*;
// use embassy_executor::Spawner;
// use embassy_stm32::can::util::calc_can_timings;
// use embassy_stm32::can::{
//     filter::Mask32, frame::Frame, util, Can, CanTx, Fifo, Rx0InterruptHandler, Rx1InterruptHandler,
//     SceInterruptHandler, StandardId, TxInterruptHandler,
// };
// use embassy_stm32::gpio::{AnyPin, Input, Pin, Pull};
// use embassy_stm32::peripherals::CAN1;
// use embassy_stm32::time::Hertz;
// use embassy_stm32::{bind_interrupts, can};
// use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
// use static_cell::StaticCell;
// use {defmt_rtt as _, panic_probe as _};

// use hyped_io::messages::*;
// use hyped_io::types::*;

// static SEND_CHANNEL: Channel<ThreadModeRawMutex, CanMessage, 50> = Channel::new();

// bind_interrupts!(struct Irqs {
//     CAN1_RX0 => Rx0InterruptHandler<CAN1>;
//     CAN1_RX1 => Rx1InterruptHandler<CAN1>;
//     CAN1_SCE => SceInterruptHandler<CAN1>;
//     CAN1_TX => TxInterruptHandler<CAN1>;
// });

// #[embassy_executor::task]
// pub async fn counter_message() {
//     let mut counter = 0;
//     loop {
//         let counter_message = CounterMessage {
//             counter: counter as u8,
//         };
//         SEND_CHANNEL
//             .send(CanMessage {
//                 board_id: 2,
//                 data: counter_message.to_can_data(),
//             })
//             .await;
//         embassy_time::Timer::after_millis(500).await;
//         counter += 1;
//     }
// }

// #[embassy_executor::task]
// pub async fn button_message(pin: AnyPin) {
//     let button = Input::new(pin, Pull::Down);
//     let mut button_id = 0;
//     loop {
//         let button_message = ButtonMessage {
//             button_id: button_id,
//             pressed: button.is_high(),
//         };
//         SEND_CHANNEL
//             .send(CanMessage {
//                 board_id: 1,
//                 data: button_message.to_can_data(),
//             })
//             .await;
//         embassy_time::Timer::after_millis(500).await;
//     }
// }

// //

// #[embassy_executor::task]
// pub async fn send_channel_task(tx: &'static mut CanTx<'static>) {
//     loop {
//         let message = SEND_CHANNEL.receive().await;
//         let frame = Frame::new_standard(message.board_id as u16, &message.data).unwrap();
//         info!("{}", frame.data());
//         tx.write(&frame).await;
//     }
// }

// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//     info!("Hello World!");

//     let mut p = embassy_stm32::init(Default::default());

//     // Pull up resistor for the RX pin to avoid the 120 Ohm termination resistor
//     // let rx_pin = Input::new(&mut p.PB4, Pull::Up);
//     // core::mem::forget(rx_pin);
//     embassy_time::Timer::after_secs(10).await;
//     static CAN: StaticCell<Can<'static>> = StaticCell::new();

//     // CAN3_RX and CAN3_TX taken from https://danieleff.github.io/STM32GENERIC/board_Nucleo_F767ZI/
//     // let can = CAN.init(Can::new(p.CAN3, p.PA8, p.PA15, Irqs));
//     let can = CAN.init(Can::new(p.CAN1, p.PD0, p.PD1, Irqs));
//     can.modify_filters()
//         .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());

//     // Use default for now until we figure out what we need
//     // http://www.bittiming.can-wiki.info/
//     // let timing = calc_can_timings(Hertz::mhz(216), 500_000).unwrap();
//     can.modify_config()
//         .set_bit_timing(can::util::NominalBitTiming {
//             prescaler: NonZeroU16::new(2).unwrap(),
//             seg1: NonZeroU8::new(13).unwrap(),
//             seg2: NonZeroU8::new(2).unwrap(),
//             sync_jump_width: NonZeroU8::new(1).unwrap(),
//         }) // http://www.bittiming.can-wiki.info/
//         .set_loopback(true);

//     can.enable().await;

//     let (tx, mut rx) = can.split();

//     static CAN_TX: StaticCell<CanTx<'static>> = StaticCell::new();
//     let tx: &mut CanTx<'_> = CAN_TX.init(tx);

//     spawner.spawn(counter_message()).unwrap();
//     spawner.spawn(send_channel_task(tx)).unwrap();
//     spawner.spawn(button_message(p.PC13.degrade())).unwrap();

//     loop {
//         let envelope = rx.read().await.unwrap();
//         let message_data = envelope.frame.data();
//         let header = message_data.get(0).unwrap();
//         match MessageID::from_u8(*header) {
//             Some(MessageID::Button) => {
//                 let button_message = ButtonMessage::from_can_data(message_data.try_into().unwrap());
//                 info!(
//                     "Recieved Button Message: Button ID: {}, Pressed: {}",
//                     button_message.button_id, button_message.pressed
//                 );
//             }
//             Some(MessageID::Counter) => {
//                 let counter_message =
//                     CounterMessage::from_can_data(message_data.try_into().unwrap());
//                 info!(
//                     "Recieved Counter Message: Counter {}",
//                     counter_message.counter
//                 );
//             }
//             None => {
//                 info!("Unknown message ID");
//             }
//         }
//     }
// }

#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::can::filter::Mask32;
use embassy_stm32::can::{
    Can, Fifo, Frame, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
    TxInterruptHandler,
};
use embassy_stm32::peripherals::CAN1;
use embassy_stm32::{bind_interrupts, Config};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());

    let mut can = Can::new(p.CAN1, p.PA11, p.PA12, Irqs);

    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all())
        .enable_bank(1, Fifo::Fifo1, Mask32::accept_all());

    can.modify_config()
        .set_loopback(true) // Receive own frames
        .set_silent(false)
        .set_bitrate(250_000);

    can.set_automatic_wakeup(true);

    can.enable().await;
    println!("CAN enabled");

    let mut i = 0;
    let mut last_read_ts = embassy_time::Instant::now();
    loop {
        // let frame = Frame::new_extended(0x123456F, &[i; 8]).unwrap();
        // info!("Writing frame");

        // _ = can.write(&frame).await;

        match can.read().await {
            Ok(envelope) => {
                let (ts, rx_frame) = (envelope.ts, envelope.frame);
                let delta = (ts - last_read_ts).as_millis();
                last_read_ts = ts;
                info!(
                    "Rx: {} {:02x} --- {}ms",
                    rx_frame.header().len(),
                    rx_frame.data()[0..rx_frame.header().len() as usize],
                    delta,
                )
            }
            Err(err) => error!("Error in frame: {}", err),
        }

        Timer::after_millis(250).await;

        i += 1;
        if i > 100 {
            break;
        }
    }
}
