use core::cell::RefCell;

use embassy_stm32::{
    adc::{Adc, AnyAdcChannel, Instance},
    can::{
        enums::{BusError, FrameCreateError, TryReadError},
        frame, Can, CanRx, CanTx, ExtendedId, Frame, Id, StandardId, TryWriteError,
    },
    gpio::{Input, Output},
    i2c::I2c,
    mode::Blocking,
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};

use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;
use hyped_can::{CanError, HypedCan, HypedCanFrame, HypedCanRx, HypedCanTx, HypedEnvelope};
use hyped_can_derive::{HypedCan, HypedCanRx, HypedCanTx};
use hyped_gpio::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_gpio_derive::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_i2c::{HypedI2c, I2cError};
use hyped_i2c_derive::HypedI2c;

#[derive(HypedAdc)]
pub struct Stm32f767ziAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    channel: AnyAdcChannel<T>,
}

#[derive(HypedGpioInputPin)]
pub struct Stm32f767ziGpioInput {
    pin: Input<'static>,
}

#[derive(HypedGpioOutputPin)]
pub struct Stm32f767ziGpioOutput {
    pin: Output<'static>,
}

#[derive(HypedI2c)]
pub struct Stm32f767ziI2c<'d> {
    i2c: &'d Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>,
}

#[derive(HypedCan)]
pub struct Stm32f767ziCan<'d> {
    can: &'d Mutex<NoopRawMutex, RefCell<&'d mut Can<'static>>>,
}

#[derive(HypedCanRx)]
pub struct Stm32f767ziCanRx<'d> {
    can: &'d Mutex<NoopRawMutex, RefCell<&'d mut CanRx<'static>>>,
}

#[derive(HypedCanTx)]
pub struct Stm32f767ziCanTx<'d> {
    can: &'d Mutex<NoopRawMutex, RefCell<&'d mut CanTx<'static>>>,
}
