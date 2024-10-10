// ADC trait used to abstract the ADC peripheral
pub trait HypedADC {
    // Read AIN value
    // Return a voltage between 0 and 1.8V  
    fn readValue(&mut self) -> Option<f32>;
}