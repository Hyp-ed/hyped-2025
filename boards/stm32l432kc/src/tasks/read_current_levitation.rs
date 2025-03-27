use crate::io::Stm32l432kcAdc;
use defmt_rtt as _;
use embassy_stm32::adc::Adc;
use embassy_stm32::adc::AdcChannel;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_sensors::{current_levitation::CurrentLevitation, SensorValueRange};

/// The update frequency of the current levitation sensor in Hz
const UPDATE_FREQUENCY: u64 = 1000;
/// Reference voltage for the current levitation sensor
const V_REF: f32 = 5.0;

/// Test task that reads the current and sends it with the Watch Sender
#[embassy_executor::task]
pub async fn read_current_levitation(
    sender: Sender<'static, CriticalSectionRawMutex, SensorValueRange<f32>, 1>,
) -> ! {
    let p = embassy_stm32::init(Default::default());
    let adc = Adc::new(p.ADC1);
    let pin = p.PA3; // Temporary pin until we know what our actual pin is

    let mut current_levitation_sensor =
        CurrentLevitation::new(Stm32l432kcAdc::new(adc, pin.degrade_adc(), V_REF));

    loop {
        sender.send(current_levitation_sensor.read());
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
