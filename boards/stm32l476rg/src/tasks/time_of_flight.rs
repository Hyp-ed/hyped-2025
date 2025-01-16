use crate::io::Stm32l476rgI2c;
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_sync::blocking_mutex::Mutex;
use hyped_sensors::time_of_flight::{TimeOfFlight, TimeOfFlightAddresses};

// Test task that continuously starts a single shot measurement and reads the result
#[embassy_executor::task]
pub async fn read_time_of_flight_range() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = Mutex::new(RefCell::new(I2c::new_blocking(
        p.I2C1,
        p.PB8,
        p.PB9,
        Hertz(100_000),
        Default::default(),
    )));
    let mut hyped_i2c = Stm32l476rgI2c::new(i2c);

    let mut time_of_flight_sensor = TimeOfFlight::new(
        &mut hyped_i2c,
        TimeOfFlightAddresses::Address29,
    )
    .expect(
        "Failed to create Time of Flight sensor. Check the wiring and I2C address of the sensor.",
    );

    loop {
        match time_of_flight_sensor.single_shot_measurement() {
            Ok(range) => {
                defmt::info!("Range {:?} mm", range)
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        }
        time_of_flight_sensor.clear_interrupts();
    }
}
