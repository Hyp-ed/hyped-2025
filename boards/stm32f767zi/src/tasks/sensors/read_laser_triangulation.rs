use crate::io::Stm32f767ziAdc;
use defmt_rtt as _;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_core::config::SENSORS_CONFIG;
use hyped_sensors::{
    laser_triangulation::{LaserTriangulation, LaserTriangulationError},
    SensorValueRange,
};

/// Test task that reads the distance by laser triangulation and sends it with the Watch Sender
#[embassy_executor::task]
pub async fn read_laser_triangulation(
    sender: Sender<'static, CriticalSectionRawMutex, SensorValueRange<f32>, 1>,
) -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1);
    let pin = p.PA3; // Temporary pin until we know what our actual pin is

    let mut laser_triangulation_sensor = LaserTriangulation::new(Stm32f767ziAdc::new(
        adc,
        pin.degrade_adc(),
        SENSORS_CONFIG.sensors.laser_triangulation.v_ref as f32,
    ));

    loop {
        match laser_triangulation_sensor.read() {
            Ok(value) => {
                // Send the value to the Watch sender
                sender.send(value);
            }
            Err(e) => match e {
                LaserTriangulationError::OutOfRange => {
                    defmt::error!("Laser triangulation sensor out of range");
                }
            },
        }
        Timer::after(Duration::from_hz(
            SENSORS_CONFIG.sensors.laser_triangulation.update_frequency as u64,
        ))
        .await;
    }
}
