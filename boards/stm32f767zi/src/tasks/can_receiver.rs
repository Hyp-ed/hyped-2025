use embassy_stm32::can::{CanRx, Id};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Sender;
use embassy_time::{Duration, Timer};
use hyped_can::HypedCanFrame;
use hyped_core::comms::messages::CanMessage;
use {defmt_rtt as _, panic_probe as _};

/// Task that receives CAN messages and sends them to the main task
#[embassy_executor::task]
pub async fn can_receiver(
    mut rx: CanRx<'static>,
    sender: Sender<'static, CriticalSectionRawMutex, CanMessage, 10>,
) {
    loop {
        let envelope = rx.read().await;
        if envelope.is_err() {
            continue;
        }
        let envelope = envelope.unwrap();
        let id = envelope.frame.id();
        let can_id = match id {
            Id::Standard(id) => id.as_raw() as u32, // 11-bit ID
            Id::Extended(id) => id.as_raw(),        // 29-bit ID
        };
        let mut data = [0u8; 8];
        data.copy_from_slice(envelope.frame.data());
        let can_frame = HypedCanFrame::new(can_id, data);

        let can_message: CanMessage = can_frame.into();

        sender.send(can_message).await;

        Timer::after(Duration::from_millis(100)).await;
    }
}
