use embassy_stm32::can::{CanTx, ExtendedId, Frame, Id};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use hyped_can::HypedCanFrame;
use hyped_communications::messages::CanMessage;

/// Channel for sending CAN messages.
pub static CAN_SEND: Channel<CriticalSectionRawMutex, CanMessage, 10> = Channel::new();

/// Task that sends CAN messages from a channel.
#[embassy_executor::task]
pub async fn can_sender(mut tx: CanTx<'static>) {
    let can_sender = CAN_SEND.receiver();

    // Clear the tx buffer
    tx.flush_all().await;
    defmt::info!("Starting...");

    loop {
        let message = can_sender.receive().await;

        defmt::info!("Sending CAN message: {:?}", message);

        let can_frame: HypedCanFrame = message.into();

        let id = Id::Extended(ExtendedId::new(can_frame.can_id).unwrap());
        let data = can_frame.data;

        let frame = Frame::new_data(id, &data).unwrap();

        tx.write(&frame).await;
        defmt::info!("CAN message sent: {:?}", frame);

        Timer::after(Duration::from_millis(10)).await;
    }
}
