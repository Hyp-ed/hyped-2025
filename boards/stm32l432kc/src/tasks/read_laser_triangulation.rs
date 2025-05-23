use crate::io::Stm32l432kcAdc;
use defmt_rtt as _;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_sensors::{
    laser_triangulation::{LaserTriangulation, LaserTriangulationError},
    SensorValueRange,
};

/// The update frequency of the laser triangulation sensor in Hz
const UPDATE_FREQUENCY: u64 = 1000;
/// Reference voltage for the laser triangulation sensor
const V_REF: f32 = 3.22;

/// Test task that reads the distance by laser triangulation and sends it with the Watch Sender
#[embassy_executor::task]
pub async fn read_laser_triangulation(
    sender: Sender<'static, CriticalSectionRawMutex, SensorValueRange<f32>, 1>,
) -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1);
    let pin = p.PA3; // Temporary pin until we know what our actual pin is

    let mut laser_triangulation_sensor =
        LaserTriangulation::new(Stm32l432kcAdc::new(adc, pin.degrade_adc(), V_REF));

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
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
