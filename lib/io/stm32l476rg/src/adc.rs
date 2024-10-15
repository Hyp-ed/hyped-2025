use embassy_stm32::{adc::Adc}
use hyped_io::Adc::HypedAdc;

pub struct Stm32l476rgAdc<'d, T> {
    adc: Adc<'d, T>,
}

impl<'d> HypedAdc for Stm32l476rgAdc<'d> {
    /// Reading from ADC requires a channel
    fn read_value(&mut self, channel: AnyAdcChannel<T>) -> Option<f32> {
        adc.blocking_read(&mut self, channel)
    }
}

impl<'d> Stm32l476rgAdc<'d> {
    /// Create a new instance of our ADC implementation for the STM32L476RG
    pub fn new(adc: Adc<'d, T>) -> Self {
        Self { adc }
    }
}

