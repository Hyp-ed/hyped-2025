use crate::io::{Stm32f767ziGpioOutput, Stm32f767ziSpi};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use heapless::Vec;
use hyped_core::config::SENSORS_CONFIG;
use hyped_sensors::optical_flow::OpticalFlow;
use hyped_spi::HypedSpiCsPin;

#[embassy_executor::task]
pub async fn read_optical_flow(
    mut hyped_spi: Stm32f767ziSpi,
    cs: HypedSpiCsPin<Stm32f767ziGpioOutput>,
    sender: Sender<'static, CriticalSectionRawMutex, Vec<f64, 2>, 1>,
) -> ! {
    let mut optical_flow = OpticalFlow::new(&mut hyped_spi, cs)
        .await
        .expect("Failed to initialize optical flow sensor.");
    defmt::info!("Optical flow sensor initialized.");

    loop {
        let flow = optical_flow.get_motion().await.unwrap();
        let optical_data: Vec<f64, 2> = Vec::from_slice(&[flow.x as f64, flow.y as f64]).unwrap();

        sender.send(optical_data);

        Timer::after(Duration::from_hz(
            SENSORS_CONFIG.sensors.optical_flow.update_frequency as u64,
        ))
        .await;
    }
}
