use core::cell::RefCell;
use embassy_stm32::adc::{Adc, AnyAdcChannel, Instance};
use embassy_stm32::can::Can;
use embassy_stm32::gpio::Input;
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;
use hyped_can::Envelope;
use hyped_can::{CanError, CanFrame, HypedCan};
use hyped_can_derive::HypedCan;
use hyped_gpio_input::HypedGpioInput;
use hyped_gpio_input_derive::HypedGpioInput;
use hyped_i2c::{HypedI2c, I2cError};
use hyped_i2c_derive::HypedI2c;

#[derive(HypedAdc)]
pub struct Stm32f767ziAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    channel: AnyAdcChannel<T>,
}

#[derive(HypedGpioInput)]
pub struct Stm32f767ziGpioInput {
    pin: Input<'static>,
}

#[derive(HypedI2c)]
pub struct Stm32f767ziI2c<'d> {
    i2c: &'d Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>,
}

#[derive(HypedCan)]
pub struct Stm32f767ziCan<'d> {
    can: &'d Mutex<NoopRawMutex, RefCell<Can<'static>>>,
}
