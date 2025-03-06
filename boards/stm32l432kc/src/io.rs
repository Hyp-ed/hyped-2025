use embassy_stm32::adc::{Adc, AnyAdcChannel, Instance};

use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;

#[derive(HypedAdc)]
pub struct Stm32l432kcAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    channel: AnyAdcChannel<T>,
    v_ref: f32,
}
