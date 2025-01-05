#![no_std]
#![no_main]

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_stm32::mode::Blocking;
use embassy_stm32::{i2c::I2c, time::Hertz};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l476rg::io::Stm32l476rgI2c;
use hyped_i2c::i2c_mux::I2cMux;
use hyped_sensors::temperature::{Temperature, TemperatureAddresses};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const MUX_ADDRESS: u8 = 0x70;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

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

    let hyped_i2c = Stm32l476rgI2c::new(i2c_bus);
    let mut i2c_mux = match I2cMux::new(hyped_i2c, channel, mux_address) {
        Ok(i2c_mux) => i2c_mux,
        Err(_) => {
            panic!("Failed to create I2C Mux. Check the wiring and the I2C address of the Mux.")
        }
    };

    let mut temperature_sensor = Temperature::new(&mut i2c_mux, temp_address).expect(
        "Failed to create temperature sensor. Check the wiring and the I2C address of the sensor.",
    );

    loop {
        match temperature_sensor.read() {
            Some(temperature) => {
                defmt::info!("Temperature ({:?}): {:?}", channel, temperature);
            }
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
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));
    defmt::info!("I2C initialized.");

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
