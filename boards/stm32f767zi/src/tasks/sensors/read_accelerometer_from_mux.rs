use crate::io::Stm32f767ziI2c;
use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_stm32::mode::Blocking;
use embassy_stm32::{i2c::I2c, time::Hertz};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use hyped_i2c::i2c_mux::I2cMux;
use hyped_sensors::{
    accelerometer::{Accelerometer, AccelerometerAddresses},
    SensorValueRange::*,
};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const MUX_ADDRESS: u8 = 0x70;

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Task that reads the accelerometer from a specific channel of an I2C Mux.
#[embassy_executor::task(pool_size = 4)]
async fn read_accelerometer_from_mux(
    i2c_bus: &'static I2c1Bus,
    accel_address: AccelerometerAddresses,
    mux_address: u8,
    channel: u8,
) -> ! {
    defmt::info!(
        "Reading accelerometer from channel {} of Mux at address 0x{:x}.",
        channel,
        mux_address
    );

    // First, we create a HypedI2c object that wraps the I2C bus.
    let hyped_i2c = Stm32f767ziI2c::new(i2c_bus);

    // Then, we create an I2C Mux object that wraps the HypedI2c object. `i2c_mux` can now be used anywhere that
    // `hyped_i2c` could be used, but it will automatically switch to the correct channel before sending any I2C commands.
    let mut i2c_mux = match I2cMux::new(hyped_i2c, channel, mux_address) {
        Ok(i2c_mux) => i2c_mux,
        Err(_) => {
            panic!("Failed to create I2C Mux. Check the wiring and the I2C address of the Mux.")
        }
    };

    // Finally, we create an Accelerometer object by passing the I2C Mux object and the I2C address of the accelerometer.
    let mut accelerometer = Accelerometer::new(&mut i2c_mux, accel_address).expect(
        "Failed to create accelerometer. Check the wiring and the I2C address of the sensor.",
    );

    loop {
        match accelerometer.read() {
            Some(accel_values) => match accel_values {
                Safe(accel_values) => {
                    defmt::info!(
                        "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (safe)",
                        accel_values.x,
                        accel_values.y,
                        accel_values.z
                    );
                }
                Warning(accel_values) => {
                    defmt::warn!(
                        "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (unsafe)",
                        accel_values.x,
                        accel_values.y,
                        accel_values.z
                    );
                }
                Critical(accel_values) => {
                    defmt::error!(
                        "Acceleration: x={:?}mg, y={:?}mg, z={:?}mg (critical)",
                        accel_values.x,
                        accel_values.y,
                        accel_values.z
                    );
                }
            },
            None => {
                defmt::info!("Failed to read acceleration values.");
            }
        }

        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(200_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the tasks.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));
    defmt::info!("I2C initialized.");

    // Spawn tasks that read the acceleration from each channel of the I2C Mux.

    spawner.must_spawn(read_accelerometer_from_mux(
        i2c_bus,
        AccelerometerAddresses::Address1d,
        MUX_ADDRESS,
        0,
    ));
    spawner.must_spawn(read_accelerometer_from_mux(
        i2c_bus,
        AccelerometerAddresses::Address1d,
        MUX_ADDRESS,
        1,
    ));
    spawner.must_spawn(read_accelerometer_from_mux(
        i2c_bus,
        AccelerometerAddresses::Address1d,
        MUX_ADDRESS,
        2,
    ));
    spawner.must_spawn(read_accelerometer_from_mux(
        i2c_bus,
        AccelerometerAddresses::Address1d,
        MUX_ADDRESS,
        3,
    ));

    loop {
        Timer::after(Duration::from_secs(100)).await;
    }
}
