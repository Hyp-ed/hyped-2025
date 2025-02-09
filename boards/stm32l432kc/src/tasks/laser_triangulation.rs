use embassy_stm32::adc::Adc;
use embassy_time::Delay;
use hyped_sensors::{laser_triangulation::LaserTriangulation, SensorValueRange::*};
use defmt_rtt as _;
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Sender,
};


/// Test task that reads the distance by laser triangulation and sends it with the Watch Sender
#[embassy_executor::task]
pub async fn read_laser_triang_distance(
    sender: Sender<'static, CriticalSectionRawMutex, SensorValueRange<f32>, 1>,
) -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1, Delay);

    let mut laser_triangulation_sensor = LaserTriangulation::new(&mut adc);

    loop {
            sender.send(laser_triangulation_sensor.read())
        }
    }