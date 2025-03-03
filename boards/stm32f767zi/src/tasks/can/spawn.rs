#[embassy_executor::task]
pub async fn can(mut can: Can<'static>) {
    defmt::info!("CAN enabled");
    join(can_receiver(can.split().1), can_sender(can.split().0)).await;
}
