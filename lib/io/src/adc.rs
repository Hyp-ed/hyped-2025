/// ADC trait used to abstract the ADC peripheral
pub trait HypedAdc {
    fn read_value(&mut self) -> u16;
}