use super::ControllerTrait;

/// Proportional-Integral (PI) controller gains
#[derive(Debug, Clone)]
pub struct PiGain {
    /// Proportional gain constant
    pub kp: f32,
    /// Integral gain constant
    pub ki: f32,
}

/// A simple Proportional-Integral (PI) controller implementation
#[derive(Debug, Clone)]
pub struct PiController {
    config: PiGain,
    integral_term: f32,
}

impl PiController {
    /// Creates a new `PiController` with the specified gain configuration
    ///
    /// Arguments:
    ///
    /// * `config` - The proportional and integral gain settings
    pub fn new(config: PiGain) -> Self {
        Self {
            config,
            integral_term: 0.0,
        }
    }
}

impl ControllerTrait for PiController {
    /// Updates the PI controller output
    ///
    /// Arguments:
    ///
    /// * `set_point` - The target value the controller should aim for
    /// * `actual` - The current measured value
    /// * `dt` - The elapsed time in microseconds since the last update
    ///
    /// Returns:
    ///
    /// The output control signal based on the error and accumulated integral
    fn update(&mut self, set_point: f32, actual: f32, dt: u64) -> f32 {
        let error = set_point - actual;
        self.integral_term += error * dt as f32;
        self.config.kp * error + self.config.ki * self.integral_term
    }
}
