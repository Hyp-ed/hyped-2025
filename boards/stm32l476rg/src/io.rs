use core::cell::RefCell;

use embassy_stm32::adc::{Adc, AnyAdcChannel, Instance};
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::spi::{self, Spi, Word};
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use heapless::Vec;

use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;
use hyped_gpio::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_gpio_derive::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_i2c::{HypedI2c, I2cError};
use hyped_i2c_derive::HypedI2c;
use hyped_spi::{HypedSpi, HypedWord, SpiError};
use hyped_spi_derive::HypedSpi;

#[derive(HypedAdc)]
pub struct Stm32l476rgAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    channel: AnyAdcChannel<T>,
}

#[derive(HypedGpioInputPin)]
pub struct Stm32l476rgGpioInput {
    pin: Input<'static>,
}

#[derive(HypedGpioOutputPin)]
pub struct Stm32l476rgGpioOutput {
    pin: Output<'static>,
}

#[derive(HypedI2c)]
pub struct Stm32l476rgI2c<'d> {
    i2c: &'d Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>,
}
