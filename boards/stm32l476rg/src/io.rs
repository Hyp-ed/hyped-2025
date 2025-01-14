use core::cell::RefCell;

use embassy_stm32::adc::{Adc, AnyAdcChannel, Instance};
use embassy_stm32::gpio::Input;
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;

use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;
use hyped_gpio::HypedGpioInput;
use hyped_gpio_derive::HypedGpioInput;
use hyped_i2c::{HypedI2c, I2cError};
use hyped_i2c_derive::HypedI2c;

#[derive(HypedAdc)]
pub struct Stm32l476rgAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    channel: AnyAdcChannel<T>,
}

#[derive(HypedGpioInput)]
pub struct Stm32l476rgGpioInput {
    pin: Input<'static>,
}

#[derive(HypedI2c)]
pub struct Stm32l476rgI2c<'d> {
    i2c: &'d Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>,
}
