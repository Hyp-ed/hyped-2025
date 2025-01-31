use embassy_time::with_timeout;
use embassy_time::Duration;
use hyped_gpio::HypedGpioOutputPin;
use hyped_i2c::HypedI2c;
use hyped_sensors::{
    time_of_flight::{TimeOfFlight, TimeOfFlightError},
    SensorValueRange,
};

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

#[derive(Debug)]
pub enum BrakeActuationFailure {
    TimeOfFlightError(TimeOfFlightError),
    SensorNotInTolerance,
    TimeoutError,
}

const BRAKE_ACTUATION_THRESHOLD: u8 = 5; // TODOLater: replace with whatever value indicates the brakes are actuated. > this value means brakes are not actuated.
const BRAKE_CHECK_TIMEOUT: Duration = Duration::from_millis(100);

/// Represents the pneumatics systems (brakes and lateral suspension) of the pod.
/// Outputs two GPIO signals, one for the brakes and one for the lateral suspension, which turn on/off a solenoid valve.
pub struct Pneumatics<'a, P: HypedGpioOutputPin, T: HypedI2c> {
    brake_state: BrakeState,
    lateral_suspension_state: LateralSuspensionState,
    brake_pin: P,
    lateral_suspension_pin: P,
    time_of_flight: TimeOfFlight<'a, T>,
}

impl<'a, P: HypedGpioOutputPin, T: HypedI2c> Pneumatics<'a, P, T> {
    pub async fn new(
        brake_pin: P,
        lateral_suspension_pin: P,
        time_of_flight: TimeOfFlight<'a, T>,
    ) -> Result<Self, BrakeActuationFailure> {
        let mut pneumatics = Pneumatics {
            brake_state: BrakeState::Engaged,
            lateral_suspension_state: LateralSuspensionState::Retracted,
            brake_pin,
            lateral_suspension_pin,
            time_of_flight,
        };

        // Engage brakes and retract lateral suspension on startup
        pneumatics.retract_lateral_suspension();
        match pneumatics.engage_brakes().await {
            Ok(_) => Ok(pneumatics),
            Err(e) => Err(e),
        }
    }

    /// Engages the brakes by setting the brake GPIO pin to low.
    /// After actuating the brakes, the time of flight sensor is checked to ensure the brakes have been actuated.
    /// If the brakes have not been actuated within the timeout period, a timeout error is returned.
    pub async fn engage_brakes(&mut self) -> Result<(), BrakeActuationFailure> {
        // Brake pin is set to low, as brakes clamp with no power,
        // and are retracted when powered.
        self.brake_pin.set_low();

        // Check that the brakes have been actuated by reading the time of flight sensor.
        match with_timeout(BRAKE_CHECK_TIMEOUT, self.check_brake_actuation()).await {
            Ok(v) => match v {
                Ok(_) => {
                    self.brake_state = BrakeState::Engaged;
                    Ok(())
                }
                Err(e) => Err(e),
            },
            // If the brakes have not been actuated within the timeout period, return a timeout error.
            Err(_) => Err(BrakeActuationFailure::TimeoutError),
        }
    }

    /// Disengages the brakes by setting the brake GPIO pin to high.
    /// After disengaging the brakes, the time of flight sensor is checked to ensure the brakes have been disengaged.
    /// If the brakes have not been disengaged within the timeout period, a timeout error is returned.
    pub async fn disengage_brakes(&mut self) -> Result<(), BrakeActuationFailure> {
        // Brake pin is set to high, as brakes retract when powered,
        // and are retracted when powered.
        self.brake_pin.set_high();

        // Check that the brakes have been disengaged by reading the time of flight sensor.
        match with_timeout(BRAKE_CHECK_TIMEOUT, self.check_brake_actuation()).await {
            Ok(v) => match v {
                Ok(_) => {
                    self.brake_state = BrakeState::Disengaged;
                    Ok(())
                }
                Err(e) => Err(e),
            },
            // If the brakes have not been disengaged within the timeout period, return a timeout error.
            Err(_) => Err(BrakeActuationFailure::TimeoutError),
        }
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
            Err(e) => return Err(BrakeActuationFailure::TimeOfFlightError(e)),
        };

        let sensor_value = match sensor_range {
            SensorValueRange::Safe(s) => s,
            SensorValueRange::Warning(w) => w,
            SensorValueRange::Critical(c) => c,
        };

        if sensor_value >= BRAKE_ACTUATION_THRESHOLD {
            return Err(BrakeActuationFailure::SensorNotInTolerance);
        }

        Ok(())
    }
}
