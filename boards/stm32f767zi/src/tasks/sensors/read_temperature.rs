use crate::{emergency, io::Stm32f767ziI2c, tasks::can::send::CAN_SEND};
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Sender,
};
use embassy_time::{Duration, Timer};
use hyped_communications::{
    boards::Board, data::CanData, emergency::Reason, measurements::MeasurementReading,
    messages::CanMessage,
};
use hyped_core::config::MeasurementId;
use hyped_sensors::temperature::{Status, Temperature, TemperatureAddresses};
use hyped_sensors::SensorValueRange;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// The update frequency of the temperature sensor in Hz
const UPDATE_FREQUENCY: u64 = 1;

/// Test task that just reads the temperature from the sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_temperature(
    i2c_bus: &'static I2c1Bus,
    this_board: Board,
    measurement_id: MeasurementId,
    latest_temperature_reading_sender: Sender<
        'static,
        CriticalSectionRawMutex,
        Option<SensorValueRange<f32>>,
        1,
    >,
) -> ! {
    let can_sender = CAN_SEND.sender();

    let mut hyped_i2c = Stm32f767ziI2c::new(i2c_bus);
    let mut temperature_sensor = Temperature::new(&mut hyped_i2c, TemperatureAddresses::Address3f)
        .expect(
        "Failed to create temperature sensor. Check the wiring and the I2C address of the sensor.",
    );

    loop {
        match temperature_sensor.check_status() {
            Status::TempOverUpperLimit => {
                defmt::error!("Temperature is over the upper limit.");
                emergency!(this_board);
            }
            Status::TempUnderLowerLimit => {
                defmt::error!("Temperature is under the lower limit.");
                emergency!(this_board);
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
                SensorValueRange::Critical(_) => {
                    emergency!(this_board);
                }
                SensorValueRange::Warning(v) => v,
                SensorValueRange::Safe(v) => v,
            };

            defmt::debug!("Sending temperature reading over CAN");
            can_sender
                .send(CanMessage::MeasurementReading(MeasurementReading::new(
                    CanData::F32(value),
                    this_board,
                    measurement_id,
                )))
                .await;
        }

        Timer::after(Duration::from_hz(UPDATE_FREQUENCY)).await;
    }
}
