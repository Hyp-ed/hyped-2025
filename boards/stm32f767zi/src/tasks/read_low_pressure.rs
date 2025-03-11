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

type Adc1Bus = Mutex<NoopRawMutex, RefCell<Adc<'static, T>>>;   // Not sure what T should be replaced with here, omitting it causes errors saying 'missing generic type'

/// The update frequency of the low pressure sensor in Hz
const UPDATE_FREQUENCY: u64 = 10;

/// Test task that just reads the pressure from the low pressure sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_low_pressure(
    adc_bus: &'static Adc1Bus,
    sender: Sender<'static, CriticalSectionRawMutex, Option<SensorValueRange<f32>>, 1>,
) -> ! {
    let mut hyped_adc = Stm32f767ziAdc::new(adc_bus);   // says a 2nd argument of type AnyAdcChannel is missing, but i looked at the function src code and it only has 1 argument?
                                                        // I can't understand what the cargo build is suggesting for me to do.

    let mut low_pressure_sensor = LowPressure::new(hyped_adc);

    loop {
        sender.send(low_pressure_sensor.read_pressure());
        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
