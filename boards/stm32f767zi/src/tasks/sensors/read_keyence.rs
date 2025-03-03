use crate::{io::Stm32f767ziGpioInput, tasks::can::send::CAN_SEND};
use embassy_stm32::gpio::Input;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use embassy_time::{Duration, Timer};
use hyped_core::comms::{
    boards::Board,
    data::{CanData, CanDataType},
    measurements::{MeasurementId, MeasurementReading},
    messages::CanMessage,
};
use hyped_sensors::keyence::Keyence;

/// Used to keep the latest temperature sensor value.
pub static CURRENT_KEYENCE_STRIPE_COUNT: Watch<CriticalSectionRawMutex, u32, 1> = Watch::new();

/// The update frequency of the Keyence sensor in Hz
const UPDATE_FREQUENCY: u64 = 10;

/// Test task that just continually updates the stripe count from the Keyence sensor (or other GPIO pin input)
#[embassy_executor::task]
pub async fn read_keyence(gpio_pin: Input<'static>, board: Board) -> ! {
    let current_stripe_count_sender = CURRENT_KEYENCE_STRIPE_COUNT.sender();
    let can_sender = CAN_SEND.sender();

    let mut keyence = Keyence::new(Stm32f767ziGpioInput::new(gpio_pin));

    keyence.update_stripe_count();
    let initial_stripe_count = keyence.get_stripe_count();
    current_stripe_count_sender.send(initial_stripe_count);
    let mut current_stripe_count = initial_stripe_count;

    loop {
        keyence.update_stripe_count();
        let new_stripe_count = keyence.get_stripe_count();

        // Check if the stripe count has changed
        if new_stripe_count != current_stripe_count {
            current_stripe_count_sender.send(new_stripe_count);
            let can_message = CanMessage::MeasurementReading(MeasurementReading::new(
                CanData::U32(new_stripe_count),
                CanDataType::U32,
                board,
                MeasurementId::KeyenceStripeCount,
            ));
            can_sender.send(can_message).await;
            current_stripe_count = new_stripe_count;
        }

        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
