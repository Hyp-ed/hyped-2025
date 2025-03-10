use crate::io::Stm32f767ziAdc;
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::adc::{Adc, AdcChannel};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Sender,
};
use embassy_time::{Duration, Timer};
use hyped_sensors::low_pressure::LowPressure;
use hyped_sensors::SensorValueRange;

type Adc1Bus = Mutex<NoopRawMutex, RefCell<Adc<'static, ADC1>>>;

/// The update frequency of the low pressure sensor in Hz
const UPDATE_FREQUENCY: u64 = 10;

/// Test task that just reads the pressure from the low pressure sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_low_pressure(
    adc_bus: &'static Adc1Bus,
    sender: Sender<'static, CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1>,
) -> ! {
    let mut hyped_adc = Stm32f767ziAdc::new(adc_bus, /* channel */);

    let mut low_pressure_sensor = LowPressure::new(&mut hyped_adc);

    loop {
        sender.send(low_pressure_sensor.read_pressure());
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
