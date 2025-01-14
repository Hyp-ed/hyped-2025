use hyped_gpio_input::GpioOutputPin;

pub enum BrakeState {
    Engaged,
    Disengaged,
}

pub enum LateralSuspensionState {
    Deployed,
    Retracted,
}

/// Represents the pneumatics systems (brakes and lateral suspension) of the pod.
/// Outputs two GPIO signals, one for the brakes and one for the lateral suspension, which turn on/off a solenoid valve.
pub struct Pneumatics<P: GpioOutputPin> {
    brakes: BrakeState,
    lateral_suspension: LateralSuspensionState,
    brake_pin: P,
    lateral_suspension_pin: P,
}

impl<P: GpioOutputPin> Pneumatics<P> {
    pub fn new(brake_pin: P, lateral_suspension_pin: P) -> Self {
        let mut pneumatics = Pneumatics {
            brakes: BrakeState::Engaged,
            lateral_suspension: LateralSuspensionState::Retracted,
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
        self.brakes = BrakeState::Engaged;

        // Brake pin is set to low, as brakes clamp with no power,
        // and are retracted when powered.
        self.brake_pin.set_low();
    }

    /// Disengages the brakes by setting the brake GPIO pin to high.
    pub fn disengage_brakes(&mut self) {
        self.brakes = BrakeState::Disengaged;

        // Brake pin is set to high, as brakes retract when powered,
        // and are retracted when powered.
        self.brake_pin.set_high();
    }

    /// Deploys the lateral suspension by setting the lateral suspension GPIO pin to high.
    pub fn deploy_lateral_suspension(&mut self) {
        self.lateral_suspension_pin.set_high();
        self.lateral_suspension = LateralSuspensionState::Deployed;
    }

    /// Retracts the lateral suspension by setting the lateral suspension GPIO pin to low.
    pub fn retract_lateral_suspension(&mut self) {
        self.lateral_suspension_pin.set_low();
        self.lateral_suspension = LateralSuspensionState::Retracted;
    }

    pub fn get_brake_state(self) -> BrakeState {
        self.brakes
    }

    pub fn get_lateral_suspension_state(self) -> LateralSuspensionState {
        self.lateral_suspension
    }
}
