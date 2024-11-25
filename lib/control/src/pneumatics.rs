use hyped_io::gpio::GpioOutputPin;

enum BrakeState {
    Engaged,
    Disengaged,
}

enum LateralSuspensionState {
    Deployed,
    Retracted,
}

struct Pneumatics<P: GpioOutputPin> {
    brakes: BrakeState,
    lateral_suspension: LateralSuspensionState,
    brake_pin: P,
    lateral_suspension_pin: P
}

impl<P: GpioOutputPin> Pneumatics<P> {
  fn new(brake_pin: P, lateral_suspension_pin: P) -> Self {

      let mut pneumatics = Pneumatics {
        brakes: BrakeState::Engaged,
        lateral_suspension: LateralSuspensionState::Retracted,
        brake_pin: brake_pin,   
        lateral_suspension_pin: lateral_suspension_pin
      };
      
      pneumatics.engage_brakes();
      pneumatics.retract_lateral_suspension();
      pneumatics
  }

  fn engage_brakes(&mut self) {
      self.brakes = BrakeState::Engaged;
      
      // Brake pin is set to low, as brakes clamp with no power,
      // and are retracted when powered. 
      self.brake_pin.set_low();
  }

  fn disengage_brakes(&mut self) {
      self.brakes = BrakeState::Disengaged;

      // Brake pin is set to high, as brakes retract when powered,
      // and are retracted when powered. 
      self.brake_pin.set_high();
  }

  fn deploy_lateral_suspension(&mut self) {
      self.lateral_suspension = LateralSuspensionState::Deployed;
      self.lateral_suspension_pin.set_high();
  }

  fn retract_lateral_suspension(&mut self) {
      self.lateral_suspension = LateralSuspensionState::Retracted;
      self.lateral_suspension_pin.set_low();
  }

  fn get_brake_state(self) -> BrakeState {
    self.brakes
  }

  fn get_lateral_suspension_state(self) -> LateralSuspensionState {
    self.lateral_suspension
  }
}
