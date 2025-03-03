use crate::{emergency, io::Stm32f767ziI2c};
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use hyped_core::{
    comms::{
        boards::Board,
        data::{CanData, CanDataType},
        measurements::{MeasurementId, MeasurementReading},
        messages::CanMessage,
        state_transition::StateTransition,
    },
    states::State,
};
use hyped_sensors::temperature::{Status, Temperature, TemperatureAddresses};
use hyped_sensors::SensorValueRange;

use super::can::CAN_SEND;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Used to keep the latest temperature sensor value.
pub static LATEST_TEMPERATURE_READING: Watch<
    CriticalSectionRawMutex,
    Option<SensorValueRange<f32>>,
    1,
> = Watch::new();

/// The update frequency of the temperature sensor in Hz
const UPDATE_FREQUENCY: u64 = 1000;

/// Test task that just reads the temperature from the sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_temperature(i2c_bus: &'static I2c1Bus, this_board: Board) -> ! {
    let latest_temperature_reading_sender = LATEST_TEMPERATURE_READING.sender();
    let can_sender = CAN_SEND.sender();

    let mut hyped_i2c = Stm32f767ziI2c::new(i2c_bus);
    let mut temperature_sensor = Temperature::new(&mut hyped_i2c, TemperatureAddresses::Address3f)
        .expect(
        "Failed to create temperature sensor. Check the wiring and the I2C address of the sensor.",
    );

    loop {
        match temperature_sensor.check_status() {
            Status::TempOverUpperLimit => {
                emergency!(this_board);
                defmt::error!("Temperature is over the upper limit.");
            }
            Status::TempUnderLowerLimit => {
                emergency!(this_board);
                defmt::error!("Temperature is under the lower limit.");
            }
            Status::Busy => {
                defmt::warn!("Temperature sensor is busy.");
            }
            Status::Unknown => {
                panic!("Could not get the status of the temperature sensor.")
            }
            Status::Ok => {}
        }

        let reading = temperature_sensor.read();

        // Send reading to the Watch
        latest_temperature_reading_sender.send(reading);

        // Send reading to CAN bus
        if let Some(reading) = reading {
            // Handle the reading based on the range
            let value = match reading {
                SensorValueRange::Critical(v) => {
                    emergency!(this_board);
                    defmt::error!("Critical temperature reading: {:?}", v);
                    v
                }
                SensorValueRange::Warning(v) => {
                    defmt::warn!("Warning temperature reading: {:?}", v);
                    v
                }
                SensorValueRange::Safe(v) => v,
            };

            let measurement_reading = MeasurementReading::new(
                CanData::F32(value),
                CanDataType::F32,
                this_board,
                MeasurementId::Temperature,
            );
            let can_message = CanMessage::MeasurementReading(measurement_reading);

            can_sender.send(can_message).await;
        }

        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
