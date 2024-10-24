use embassy_stm32::{adc::Adc, adc::AnyAdcChannel, adc::Instance};
use hyped_io::adc::HypedAdc;

pub struct Stm32l476rgAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    pin: AnyAdcChannel<T>,
}

impl<'d, T: Instance> HypedAdc for Stm32l476rgAdc<'d, T> {
    /// Reading from ADC requires the peripheral pin to read from
    fn read_value(&mut self) -> u16 {
        self.adc.blocking_read(&mut self.pin)
    }
}

impl<'d, T: Instance> Stm32l476rgAdc<'d, T> {
    /// Create a new instance of our ADC implementation for the STM32L476RG
    pub fn new(adc: Adc<'d, T>, pin: AnyAdcChannel<T>) -> Self {
        Self { adc, pin }
    }
}

