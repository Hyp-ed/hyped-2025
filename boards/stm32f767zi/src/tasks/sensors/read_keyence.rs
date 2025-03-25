use crate::{io::Stm32f767ziGpioInput, tasks::can::send::CAN_SEND};
use embassy_stm32::gpio::Input;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_communications::{
    boards::Board, data::CanData, measurements::MeasurementReading, messages::CanMessage,
};
use hyped_core::{config::MeasurementId, types::DigitalSignal};
use hyped_sensors::keyence::Keyence;

/// The update frequency of the Keyence sensor in Hz
const UPDATE_FREQUENCY: u64 = 10;

/// Test task that just continually updates the stripe count from the Keyence sensor (or other GPIO pin input)
#[embassy_executor::task]
pub async fn read_keyence(
    gpio_pin: Input<'static>,
    this_board: Board,
    measurement_id: MeasurementId,
    latest_stripe_count_sender: Sender<'static, CriticalSectionRawMutex, u32, 1>,
) -> ! {
    let can_sender = CAN_SEND.sender();

    let mut keyence = Keyence::new(Stm32f767ziGpioInput::new(gpio_pin), DigitalSignal::High);

    keyence.update_stripe_count();
    latest_stripe_count_sender.send(keyence.get_stripe_count());

    loop {
        keyence.update_stripe_count();
        let new_stripe_count = keyence.get_stripe_count();

        latest_stripe_count_sender.send_if_modified(|old_stripe_count| {
            if Some(new_stripe_count) != *old_stripe_count {
                *old_stripe_count = Some(new_stripe_count);
                true
            } else {
                false
            }
        });

        // Send stripe count to CAN bus
        can_sender
            .send(CanMessage::MeasurementReading(MeasurementReading::new(
                CanData::U32(new_stripe_count),
                this_board,
                measurement_id,
            )))
            .await;

        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
