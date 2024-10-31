use hyped_core::types::DigitalSignal;
use hyped_io::gpio::HypedGpioPin;

/// Keyence represents a Keyence sensor which keeps track of the number of stripes that have passed
/// by the sensor. The Keyence sensor is connected to a GPIO pin which reads a high signal when a
/// stripe is detected and a low signal when no stripe is detected. The stripe count is updated
/// whenever the signal changes from low to high (positive edge).
pub struct Keyence<T: HypedGpioPin> {
    /// The number of stripes that have passed by the sensor.
    stripe_count: u32,
    /// The last signal that was read from the sensor.
    last_signal: DigitalSignal,
    gpio: T,
}

impl<T: HypedGpioPin> Keyence<T> {
    /// Creates a new Keyence sensor with an initial stripe count of 0 and a last signal of low.
    pub fn new(gpio: T) -> Keyence<T> {
        Keyence {
            stripe_count: 0,
            last_signal: DigitalSignal::Low,
            gpio,
        }
    }

    /// Returns the number of stripes that have passed by the sensor.
    pub fn get_stripe_count(&self) -> u32 {
        self.stripe_count
    }

    /// Increments the stripe count if the signal changes from low to high (positive edge).
    pub fn update_stripe_count(&mut self) {
        let current_signal = DigitalSignal::from_bool(self.gpio.is_high());
        if current_signal == DigitalSignal::High && self.last_signal == DigitalSignal::Low {
            self.stripe_count += 1;
        }
        self.last_signal = current_signal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use heapless::Vec;
    use hyped_io::gpio::mock_gpio::MockGpio;

    #[test]
    fn test_keyence_new() {
        let gpio = MockGpio::new(Vec::from_slice(&[false]).unwrap());
        let keyence = Keyence::new(gpio);
        assert_eq!(keyence.get_stripe_count(), 0);
    }

    #[test]
    fn test_keyence_update_stripe_count_low_to_high() {
        let gpio = MockGpio::new(Vec::from_slice(&[false, true]).unwrap());
        let mut keyence = Keyence::new(gpio);

        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 0);
        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 1);
    }

    #[test]
    fn test_keyence_update_stripe_count_high_to_low() {
        let gpio = MockGpio::new(Vec::from_slice(&[true, false]).unwrap());
        let mut keyence = Keyence::new(gpio);

        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 1);
        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 1);
    }

    #[test]
    fn test_keyence_update_stripe_count_high_to_high() {
        let gpio = MockGpio::new(Vec::from_slice(&[true, true]).unwrap());
        let mut keyence = Keyence::new(gpio);

        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 1);
        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 1);
    }

    #[test]
    fn test_keyence_update_stripe_count_low_to_low() {
        let gpio = MockGpio::new(Vec::from_slice(&[false, false]).unwrap());
        let mut keyence = Keyence::new(gpio);

        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 0);
        keyence.update_stripe_count();
        assert_eq!(keyence.get_stripe_count(), 0);
    }
}
