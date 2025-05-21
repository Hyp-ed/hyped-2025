use crate::io::Stm32l476rgI2c;
use core::cell::RefCell;
use defmt_rtt as _;
use embassy_stm32::{i2c::I2c, mode::Blocking, time::Hertz};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use hyped_sensors::{
    time_of_flight::{TimeOfFlight, TimeOfFlightAddresses},
    SensorValueRange::*,
};
use static_cell::StaticCell;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

// Test task that continuously starts a single shot measurement and reads the result
#[embassy_executor::task]
pub async fn read_time_of_flight_range() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));

    let mut hyped_i2c = Stm32l476rgI2c::new(i2c_bus);

    let mut time_of_flight_sensor = TimeOfFlight::new(
        &mut hyped_i2c,
        TimeOfFlightAddresses::Address29,
    )
    .expect(
        "Failed to create Time of Flight sensor. Check the wiring and I2C address of the sensor.",
    );

    loop {
        match time_of_flight_sensor.single_shot_measurement() {
            Ok(range) => match range {
                Safe(range) => {
                    defmt::info!("Range: {}mm", range);
                }
                Warning(range) => {
                    defmt::info!("Range: {}mm", range);
                }
                Critical(range) => {
                    defmt::info!("Range: {}mm", range);
                }
            },
            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
}
