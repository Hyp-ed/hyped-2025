use crate::io::Stm32f767ziI2c;
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
use heapless::Vec;
use hyped_i2c::i2c_mux::I2cMux;
use hyped_sensors::temperature::{Status, Temperature, TemperatureAddresses};
use hyped_sensors::SensorValueRange;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

const NUM_TEMPERATURE_SENSORS: usize = 24;
type TemperatureMuxReadings = Vec<Option<SensorValueRange<f32>>, NUM_TEMPERATURE_SENSORS>;

/// Test task that just reads the temperature from the sensor and prints it to the console
#[embassy_executor::task]
pub async fn read_temperature(
    i2c_bus: &'static I2c1Bus,
    sender: Sender<'static, CriticalSectionRawMutex, TemperatureMuxReadings, 1>,
) -> ! {
    // Create all the I2C Muxes
    let mut i2c_muxes: Vec<I2cMux<Stm32f767ziI2c<'_>>, NUM_TEMPERATURE_SENSORS> = Vec::new();

    for i in 0..NUM_TEMPERATURE_SENSORS as u8 {
        let mux_address = match i {
            0..=7 => MUX_1_ADDRESS,
            8..=15 => MUX_2_ADDRESS,
            16..=23 => MUX_3_ADDRESS,
            _ => panic!("Invalid temperature sensor index."),
        };

        let channel = i % 8;

        let i2c_mux = match I2cMux::new(Stm32f767ziI2c::new(i2c_bus), channel, mux_address) {
            Ok(i2c_mux) => i2c_mux,
            Err(_) => {
                panic!("Failed to create I2C Mux. Check the wiring and the I2C address of the Mux.")
            }
        };

        match i2c_muxes.push(i2c_mux) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to add I2C Mux to the vector.");
            }
        }
    }

    // Create all the temperature sensors
    let mut temperature_sensors: Vec<
        Temperature<'_, I2cMux<Stm32f767ziI2c<'_>>>,
        NUM_TEMPERATURE_SENSORS,
    > = i2c_muxes.iter_mut().map(|i2c_mux| {
        Temperature::new(i2c_mux, TemperatureAddresses::Address3f)
            .expect("Failed to create temperature sensor. Check the wiring and the I2C address of the sensor.")
    }).collect();

    loop {
        let mut readings: TemperatureMuxReadings = Vec::new();

        // Read from all the temperature sensors
        for i in 0..NUM_TEMPERATURE_SENSORS as u8 {
            let temperature_sensor = &mut temperature_sensors[i as usize];

            match temperature_sensor.check_status() {
                Status::TempOverUpperLimit => {
                    defmt::error!("Temperature is over the upper limit.");
                }
                Status::TempUnderLowerLimit => {
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

            readings
                .push(temperature_sensor.read())
                .expect("Failed to add temperature reading to the vector.");
        }

        // Send this round of readings
        sender.send(readings);
    }
}

const MUX_1_ADDRESS: u8 = 0x70;
const MUX_2_ADDRESS: u8 = 0x71;
const MUX_3_ADDRESS: u8 = 0x72;
