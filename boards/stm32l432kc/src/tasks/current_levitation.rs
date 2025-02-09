use embassy_stm32::adc::Adc;
use embassy_time::Delay;
use hyped_sensors::{current_levitation::CurrentLevitation, SensorValueRange::*};
use defmt_rtt as _;
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Sender,
};


/// Test task that reads the current and sends it with the Watch Sender
#[embassy_executor::task]
pub async fn read_current_levitation(
    sender: Sender<'static, CriticalSectionRawMutex, SensorValueRange<f32>, 1>,
) -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1, Delay);

    let mut current_levitation_sensor = CurrentLevitation::new(&mut adc);

    loop {
            sender.send(current_levitation_sensor.read())
        }
    }
