use embassy_stm32::{adc::AnyAdcChannel}

// ADC trait used to abstract the ADC peripheral
pub trait HypedAdc {
    fn read_value(&mut self, channel: AnyAdcChannel<T>) -> Option<u16>;
}