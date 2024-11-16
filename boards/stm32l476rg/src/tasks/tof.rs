use crate::io::i2c::Stm32l476rgI2c;
use defmt_rtt as _;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use hyped_sensors::tof::{TimeOfFlight, ToFAddresses};

// Test task that continuously starts a single shot measurement and reads the result
#[embassy_executor::task]
pub async fn read_tof_range() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());
    let mut hyped_i2c = Stm32l476rgI2c::new(i2c);

    let mut tof_sensor = TimeOfFlight::new(&mut hyped_i2c, ToFAddresses::Address29).expect(
        "Failed to create Time of Flight sensor. Check the wiring and I2C address of the sensor.",
    );

    loop {
        tof_sensor.start_ss_measure();

        tof_sensor.poll_range();

        match tof_sensor.read_range() {
            Some(range) => {
                defmt::info!("Range: {:?}", range)
            }
            None => {
                defmt::info!("Failed to read range")
            }
        }
    }
}
