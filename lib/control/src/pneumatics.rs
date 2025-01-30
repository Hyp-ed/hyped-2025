use hyped_gpio::HypedGpioOutputPin;
use hyped_sensors::{time_of_flight::{TimeOfFlight, TimeOfFlightError}, SensorValueRange};
use hyped_i2c::HypedI2c;
use embassy_time::with_timeout;
use embassy_time::Duration;

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
pub enum BrakeActuationFailure {
  TimeOfFlightError(TimeOfFlightError),
  SensorNotInTolerance,
  TimeoutError
}


const BRAKE_ACTUATION_THRESHOLD: u8 = 0;
const BRAKE_CHECK_TIMEOUT: Duration = Duration::from_millis(100); //100ms

/// Represents the pneumatics systems (brakes and lateral suspension) of the pod.
/// Outputs two GPIO signals, one for the brakes and one for the lateral suspension, which turn on/off a solenoid valve.
pub struct Pneumatics<'a, P: HypedGpioOutputPin, T: HypedI2c> {
    brake_state: BrakeState,
    lateral_suspension_state: LateralSuspensionState,
    brake_pin: P,
    lateral_suspension_pin: P,
    time_of_flight: TimeOfFlight<'a, T>
}

impl<'a, P: HypedGpioOutputPin, T: HypedI2c> Pneumatics<'a, P, T> {
    pub async fn new(brake_pin: P, lateral_suspension_pin: P, time_of_flight: TimeOfFlight<'a, T>) -> Result<Self, BrakeActuationFailure> {
        let mut pneumatics = Pneumatics {
            brake_state: BrakeState::Engaged,
            lateral_suspension_state: LateralSuspensionState::Retracted,
            brake_pin,
            lateral_suspension_pin,
            time_of_flight
        };

        // Engage brakes and retract lateral suspension on startup        
        pneumatics.retract_lateral_suspension();
        match pneumatics.engage_brakes().await {
          Ok(v) => Ok(pneumatics),
          Err(e) => Err(e)
        }
        
    }

    /// Engages the brakes by setting the brake GPIO pin to low.
    pub async fn engage_brakes(&mut self) -> Result<(), BrakeActuationFailure> {
        self.brake_state = BrakeState::Engaged;

        // Brake pin is set to low, as brakes clamp with no power,
        // and are retracted when powered.
        self.brake_pin.set_low();

        match with_timeout(BRAKE_CHECK_TIMEOUT, self.check_brake_actuation()).await {
          Ok(v) => match v {
            Ok(v) => Ok(v),
            Err(e) => Err(e)
          },
          Err(e) => Err(BrakeActuationFailure::TimeoutError),
        }

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

    async fn check_brake_actuation(&mut self) -> Result<(), BrakeActuationFailure> {
      let sensor_result = self.time_of_flight.single_shot_measurement();
      
      let sensor_range = match sensor_result {
        Ok(v) => v, 
        Err(e) => return Err(BrakeActuationFailure::TimeOfFlightError(e))
      };
      
      // todo: Check this is correct.
      let sensor_value = match sensor_range {
          SensorValueRange::Safe(s) => s,
          SensorValueRange::Warning(w) => w,
          SensorValueRange::Critical(c) => c
      };

      if sensor_value >= BRAKE_ACTUATION_THRESHOLD {  // not good
        return Err(BrakeActuationFailure::SensorNotInTolerance);
      } else {
        return Ok(());
      }
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

        let mut pneumatics = Pneumatics::new(brake_pin, lateral_suspension_pin).await.unwrap(); //TODOLater: add mock tof

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