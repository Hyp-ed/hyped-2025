use crate::io::i2c::Stm32l476rgI2c;
use defmt_rtt as _;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use hyped_sensors::LedDriver::{LedDriver, LedDriverAddresses};

#[embassy_executor::task]
pub async fn write_led() -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());
    let mut hyped_i2c = Stm32l476rgI2c::new(i2c);

    let mut led_driver_sensor = LedDriver::new(&mut hyped_i2c, LedDriverAddresses::Address30)
        .expect("Failed to create LED driver instance. Check wiring and I2C address of component.");

    const DRIVER_ADDRESSES: [u8; 8] = [0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F, 0xFF];
    let mut index = 0;

    loop {
        // flash each LED in sequence
        if index == 8 {
            // reset LED Driver after flashing 8th test LED
            match led_driver_sensor.reset() {
                Ok(_) => (),
                Err(_) => {
                    defmtt::error!("Failed to reset LED Driver");
                }
            };
        }
        match led_driver_sensor.set_led_colour(
            LED_CONFIG0,
            DRIVER_ADDRESSES[index],
            0xFF, 
            0xFF, 
            0xFF, 
            0xFF
        ) {
            Ok(_) => (),
            Err(_) => {
                defmtt::error!("Failed to set colour of LED.");
            }
        };
        index += 1;
    }
}