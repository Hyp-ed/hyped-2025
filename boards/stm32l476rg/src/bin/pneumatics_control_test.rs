#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::gpio::{Level, Speed};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32l476rg::io::Stm32l476rgGpioOutput;
use hyped_control::pneumatics::Pneumatics;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());

    // Create two GPIO output pins for the solenoids
    let brakes_solenoid_pin =
        Stm32l476rgGpioOutput::new(Output::new(p.PA1, Level::Low, Speed::Low));
    let suspension_solenoid_pin =
        Stm32l476rgGpioOutput::new(Output::new(p.PA2, Level::Low, Speed::Low));

    // Create pneumatics control object
    let mut pneumatics = Pneumatics::new(brakes_solenoid_pin, suspension_solenoid_pin);

    loop {
        // Simple test to open and close the brakes solenoid
        defmt::info!("Opening brakes solenoid...");
        pneumatics.engage_brakes();
        Timer::after(Duration::from_millis(1000)).await;
        defmt::info!("Closing brakes solenoid...");
        pneumatics.disengage_brakes();
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
