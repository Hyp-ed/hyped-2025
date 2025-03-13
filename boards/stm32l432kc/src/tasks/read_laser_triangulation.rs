use crate::io::Stm32l432kcAdc;
use embassy_stm32::adc::Adc;
use embassy_stm32::adc::AdcChannel;
use embassy_time::{Duration, Timer};
use hyped_sensors::{laser_triangulation::LaserTriangulation, SensorValueRange::*};
use defmt_rtt as _;
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Sender,
};

/// The update frequency of the laser triangulation sensor in Hz
const UPDATE_FREQUENCY: u64 = 1000;
/// Reference voltage for the laser triangulation sensor
const V_REF: f32 = 5.0;

/// Test task that reads the distance by laser triangulation and sends it with the Watch Sender
#[embassy_executor::task]
pub async fn read_laser_triangulation_distance(
    sender: Sender<'static, CriticalSectionRawMutex, SensorValueRange<f32>, 1>,
) -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1);
    let pin = p.PA3; // Temporary pin until we know what our actual pin is
    
    let hyped_adc = Stm32f767ziAdc::new(adc, pin.degrade_adc(), V_REF);
    let mut laser_triangulation_sensor = LaserTriangulation::new(hyped_adc);

    loop {
            sender.send(laser_triangulation_sensor.read());
            Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
        }
    }