#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pin, Pull};
use embassy_time::{Duration, Timer};
use hyped_io::gpio::GpioPin;
use hyped_sensors::keyence::Keyence;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn keyence() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut keyence = Keyence::new(Stm32l476rgGpio::new(Input::new(p.PC13, Pull::Down)));

    loop {
        keyence.update_stripe_count().unwrap();
        defmt::info!("Stripe count: {}", keyence.get_stripe_count());
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    spawner.spawn(keyence()).unwrap();

    // Some other tasks for navigation here...

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

// Define a struct that implements the GpioPin trait for this board (STM32L476RG).

pub struct Stm32l476rgGpio<P: Pin> {
    pin: Input<'static, P>,
}

impl<P: Pin> GpioPin for Stm32l476rgGpio<P> {
    fn is_high(&mut self) -> bool {
        self.pin.is_high()
    }
}

impl<P: Pin> Stm32l476rgGpio<P> {
    pub fn new(pin: Input<'static, P>) -> Self {
        Self { pin }
    }
}
