use crate::io::i2c::Stm32l476rgI2c;
use defmt_rtt as _;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use hyped_sensors::time_of_flight::{TimeOfFlight, TimeOfFlightAddresses};

// Test task that continuously starts a single shot measurement and reads the result
#[embassy_executor::task]
pub async fn read_time_of_flight_range() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());
    let mut hyped_i2c = Stm32l476rgI2c::new(i2c);

    let mut time_of_flight_sensor = TimeOfFlight::new(
        &mut hyped_i2c,
        TimeOfFlightAddresses::Address29,
    )
    .expect(
        "Failed to create Time of Flight sensor. Check the wiring and I2C address of the sensor.",
    );

    loop {
        // defmt::info!("Starting single shot measurement");
        // defmt::info!("Polling for range");
        time_of_flight_sensor.start_single_shot_measurement();
        // defmt::info!("Started single shot measurement");

        match time_of_flight_sensor.read_range() {
            Some(range) => {
                defmt::info!("Range: {:?} mm", range)
            }
            None => {
                defmt::info!("Failed to read range")
            }
        }
        time_of_flight_sensor.clear_interrupts();
        // defmt::info!("Cleared Interrupts!")
    }
}
