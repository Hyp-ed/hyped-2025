use core::cell::RefCell;

use embassy_stm32::{
    adc::{Adc, AnyAdcChannel, Instance},
    can::{
        enums::{BusError, FrameCreateError, TryReadError},
        frame, Can, CanRx, CanTx, ExtendedId, Frame, Id, StandardId, TryWriteError,
    },
    gpio::{Input, Output},
    i2c::{self, I2c},
    mode::Blocking,
    spi::{self, Spi},
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use heapless::Vec;

use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;
use hyped_can::{CanError, HypedCan, HypedCanFrame, HypedCanRx, HypedCanTx, HypedEnvelope};
use hyped_can_derive::{HypedCan, HypedCanRx, HypedCanTx};
use hyped_gpio::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_gpio_derive::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_i2c::{HypedI2c, I2cError};
use hyped_i2c_derive::HypedI2c;
use hyped_spi::{HypedSpi, SpiError};
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

#[derive(HypedSpi)]
pub struct Stm32l476rgSpi {
    spi: Spi<'static, Blocking>,
}

#[derive(HypedCan)]
pub struct Stm32l476rgCan<'d> {
    can: &'d Mutex<NoopRawMutex, RefCell<&'d mut Can<'static>>>,
}

#[derive(HypedCanRx)]
pub struct Stm32l476rgCanRx<'d> {
    can: &'d Mutex<NoopRawMutex, RefCell<&'d mut CanRx<'static>>>,
}

#[derive(HypedCanTx)]
pub struct Stm32l476rgCanTx<'d> {
    can: &'d Mutex<NoopRawMutex, RefCell<&'d mut CanTx<'static>>>,
}
