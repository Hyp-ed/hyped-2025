#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{i2c::I2c, mode::Blocking, time::Hertz};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l476rg::io::Stm32l476rgI2c;
use hyped_i2c::i2c_mux::I2cMux;
use hyped_sensors::{
    temperature::{Temperature, TemperatureAddresses},
    SensorValueRange::*,
};
use panic_probe as _;
use static_cell::StaticCell;

const MUX_ADDRESS: u8 = 0x70;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Task that reads the temperature from a specific channel of an I2C Mux.
#[embassy_executor::task(pool_size = 4)]
async fn read_temperature_from_mux(
    i2c_bus: &'static I2c1Bus,
    temp_address: TemperatureAddresses,
    mux_address: u8,
    channel: u8,
) -> ! {
    defmt::info!(
        "Reading temperature from channel {} of Mux at address 0x{:x}.",
        channel,
        mux_address
    );

    // First, we create a HypedI2c object that wraps the I2C bus.
    let hyped_i2c = Stm32l476rgI2c::new(i2c_bus);

    // Then, we create an I2C Mux object that wraps the HypedI2c object. `i2c_mux` can now be used anywhere that
    // `hyped_i2c` could be used, but it will automatically switch to the correct channel before sending any I2C commands.
    let mut i2c_mux = match I2cMux::new(hyped_i2c, channel, mux_address) {
        Ok(i2c_mux) => i2c_mux,
        Err(_) => {
            panic!("Failed to create I2C Mux. Check the wiring and the I2C address of the Mux.")
        }
    };

    // Finally, we create a Temperature object by passing the I2C Mux object and the I2C address of the temperature sensor.
    let mut temperature_sensor = Temperature::new(&mut i2c_mux, temp_address).expect(
        "Failed to create temperature sensor. Check the wiring and the I2C address of the sensor.",
    );

    loop {
        match temperature_sensor.read() {
            Some(temperature) => match temperature {
                Safe(temp) => {
                    defmt::info!("Temperature: {}°C (safe)", temp);
                }
                Warning(temp) => {
                    defmt::warn!("Temperature: {}°C (warning)", temp);
                }
                Critical(temp) => {
                    defmt::error!("Temperature: {}°C (critical)", temp);
                }
            },
            None => {
                defmt::info!("Failed to read temperature.");
            }
        }

        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the tasks.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));
    defmt::info!("I2C initialized.");

    // Spawn tasks that read the temperature from each channel of the I2C Mux.

    spawner.must_spawn(read_temperature_from_mux(
        i2c_bus,
        TemperatureAddresses::Address3f,
        MUX_ADDRESS,
        0,
    ));
    spawner.must_spawn(read_temperature_from_mux(
        i2c_bus,
        TemperatureAddresses::Address3f,
        MUX_ADDRESS,
        1,
    ));
    spawner.must_spawn(read_temperature_from_mux(
        i2c_bus,
        TemperatureAddresses::Address3f,
        MUX_ADDRESS,
        2,
    ));
    spawner.must_spawn(read_temperature_from_mux(
        i2c_bus,
        TemperatureAddresses::Address3f,
        MUX_ADDRESS,
        3,
    ));

    loop {
        Timer::after(Duration::from_secs(100)).await;
    }
}
