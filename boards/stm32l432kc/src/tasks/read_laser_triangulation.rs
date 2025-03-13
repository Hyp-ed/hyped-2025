use embassy_stm32::adc::Adc;
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


/// Test task that reads the distance by laser triangulation and sends it with the Watch Sender
#[embassy_executor::task]
pub async fn read_laser_triangulation_distance(
    sender: Sender<'static, CriticalSectionRawMutex, SensorValueRange<f32>, 1>,
) -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1, Delay);

    let mut laser_triangulation_sensor = LaserTriangulation::new(&mut adc);

    loop {
            sender.send(laser_triangulation_sensor.read());
            Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
        }
    }