#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::{Level, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::mode::Blocking;
use embassy_stm32::time::Hertz;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l476rg::io::{Stm32l476rgGpioOutput, Stm32l476rgI2c};
use hyped_control::pneumatics::Pneumatics;
use hyped_sensors::time_of_flight::{TimeOfFlight, TimeOfFlightAddresses};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    // Create two GPIO output pins for the solenoids
    let brakes_solenoid_pin =
        Stm32l476rgGpioOutput::new(Output::new(p.PA1, Level::Low, Speed::Low));
    let suspension_solenoid_pin =
        Stm32l476rgGpioOutput::new(Output::new(p.PA2, Level::Low, Speed::Low));

    // Create I2C bus for the time-of-flight sensor
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));

    let mut hyped_i2c = Stm32l476rgI2c::new(i2c_bus);

    // Create time-of-flight sensor
    let time_of_flight_sensor = TimeOfFlight::new(
        &mut hyped_i2c,
        TimeOfFlightAddresses::Address29,
    )
    .expect(
        "Failed to create Time of Flight sensor. Check the wiring and I2C address of the sensor.",
    );

    // Create pneumatics control object
    let mut pneumatics = Pneumatics::new(
        brakes_solenoid_pin,
        suspension_solenoid_pin,
        time_of_flight_sensor,
    )
    .await
    .expect("Failed to create pneumatics control object.");

    loop {
        // Simple test to open and close the brakes solenoid
        defmt::info!("Opening brakes solenoid...");
        pneumatics.engage_brakes().await.expect(
            "Failed to engage brakes. Check the wiring and GPIO pin of the brakes solenoid.",
        );
        Timer::after(Duration::from_millis(1000)).await;
        defmt::info!("Closing brakes solenoid...");
        pneumatics.disengage_brakes().await.expect(
            "Failed to disengage brakes. Check the wiring and GPIO pin of the brakes solenoid.",
        );
        Timer::after(Duration::from_millis(1000)).await;

        // Simple test to open and close the suspension solenoid
        defmt::info!("Opening suspension solenoid...");
        pneumatics.deploy_lateral_suspension();
        Timer::after(Duration::from_millis(1000)).await;
        defmt::info!("Closing suspension solenoid...");
        pneumatics.retract_lateral_suspension();
        Timer::after(Duration::from_millis(1000)).await;
    }

    // Note: In reality, the pneumatics would be responding to commands/messages from the main control loop
}
