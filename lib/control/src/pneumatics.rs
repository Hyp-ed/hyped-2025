use hyped_gpio::GpioOutputPin;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BrakeState {
    Engaged,
    Disengaged,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LateralSuspensionState {
    Deployed,
    Retracted,
}

/// Represents the pneumatics systems (brakes and lateral suspension) of the pod.
/// Outputs two GPIO signals, one for the brakes and one for the lateral suspension, which turn on/off a solenoid valve.
pub struct Pneumatics<P: GpioOutputPin> {
    brake_state: BrakeState,
    lateral_suspension_state: LateralSuspensionState,
    brake_pin: P,
    lateral_suspension_pin: P,
}

impl<P: GpioOutputPin> Pneumatics<P> {
    pub fn new(brake_pin: P, lateral_suspension_pin: P) -> Self {
        let mut pneumatics = Pneumatics {
            brake_state: BrakeState::Engaged,
            lateral_suspension_state: LateralSuspensionState::Retracted,
            brake_pin,
            lateral_suspension_pin,
        };

        // Engage brakes and retract lateral suspension on startup
        pneumatics.engage_brakes();
        pneumatics.retract_lateral_suspension();
        pneumatics
    }

    /// Engages the brakes by setting the brake GPIO pin to low.
    pub fn engage_brakes(&mut self) {
        self.brake_state = BrakeState::Engaged;

        // Brake pin is set to low, as brakes clamp with no power,
        // and are retracted when powered.
        self.brake_pin.set_low();
    }

    /// Disengages the brakes by setting the brake GPIO pin to high.
    pub fn disengage_brakes(&mut self) {
        self.brake_state = BrakeState::Disengaged;

        // Brake pin is set to high, as brakes retract when powered,
        // and are retracted when powered.
        self.brake_pin.set_high();
    }

    /// Deploys the lateral suspension by setting the lateral suspension GPIO pin to high.
    pub fn deploy_lateral_suspension(&mut self) {
        self.lateral_suspension_pin.set_high();
        self.lateral_suspension_state = LateralSuspensionState::Deployed;
    }

    /// Retracts the lateral suspension by setting the lateral suspension GPIO pin to low.
    pub fn retract_lateral_suspension(&mut self) {
        self.lateral_suspension_pin.set_low();
        self.lateral_suspension_state = LateralSuspensionState::Retracted;
    }

    pub fn get_brake_state(&self) -> BrakeState {
        self.brake_state
    }

    pub fn get_lateral_suspension_state(&self) -> LateralSuspensionState {
        self.lateral_suspension_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyped_gpio::mock_gpio::MockGpioOutputPin;

    #[test]
    fn test_pneumatics() {
        let brake_pin = MockGpioOutputPin::new();
        let lateral_suspension_pin = MockGpioOutputPin::new();

        let mut pneumatics = Pneumatics::new(brake_pin, lateral_suspension_pin);

        // Check that the brakes are engaged and the lateral suspension is retracted on startup
        assert_eq!(pneumatics.get_brake_state(), BrakeState::Engaged);
        assert_eq!(
            pneumatics.get_lateral_suspension_state(),
            LateralSuspensionState::Retracted
        );

        // Disengage brakes
        pneumatics.disengage_brakes();
        assert_eq!(pneumatics.get_brake_state(), BrakeState::Disengaged);

        // Engage brakes
        pneumatics.engage_brakes();
        assert_eq!(pneumatics.get_brake_state(), BrakeState::Engaged);

        // Deploy lateral suspension
        pneumatics.deploy_lateral_suspension();
        assert_eq!(
            pneumatics.get_lateral_suspension_state(),
            LateralSuspensionState::Deployed
        );

        // Retract lateral suspension
        pneumatics.retract_lateral_suspension();
        assert_eq!(
            pneumatics.get_lateral_suspension_state(),
            LateralSuspensionState::Retracted
        );
    }
}
