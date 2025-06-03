use super::ControllerTrait;
use crate::config::SAMPLING_PERIOD;

/// A filtered derivative component for use in PID controllers.
/// Implements a first-order low-pass filter to smooth the derivative term.
/// This helps to reduce noise and improve stability in the control output, as
/// raw derivative calculations can be sensitive to high-frequency noise in the input signal.
#[derive(Debug, Clone)]
struct FilteredDerivative {
    b0: f32,
    b1: f32,
    a1: f32,
    prev_err: f32,
    prev_output: f32,
}

impl FilteredDerivative {
    /// Creates a new filtered derivative with the given derivative gain `kd`,
    /// time constant `tau`, and sampling period
    pub fn new(kd: f32, tau: f32, sampling_period: u64) -> Self {
        let sp = sampling_period as f32;
        let denom = sp * (tau + sp) + 2.0;

        Self {
            b0: 2.0 * kd / denom,
            b1: -2.0 * kd / denom,
            a1: 2.0 * sp / denom,
            prev_err: 0.0,
            prev_output: 0.0,
        }
    }

    /// Updates the filter with the current error and returns the smoothed derivative value
    pub fn update(&mut self, error: f32) -> f32 {
        let output = self.b0 * error + self.b1 * self.prev_err - self.a1 * self.prev_output;
        self.prev_err = error;
        self.prev_output = output;
        output
    }
}

/// Configuration values for a PID controller
#[derive(Debug, Clone)]
pub struct PidGain {
    /// Proportional gain
    pub kp: f32,
    /// Integral gain
    pub ki: f32,
    /// Derivative gain
    pub kd: f32,
    /// Gain applied to the reference value in the proportional term
    pub p_reference_gain: f32,
    /// Gain applied to the reference value in the derivative term
    pub d_reference_gain: f32,
    /// Filter coefficient used to smooth the derivative
    pub filter_coefficient: f32,
}

/// A PID controller implementation with filtered derivative term and reference gains
#[derive(Debug, Clone)]
pub struct PidController {
    config: PidGain,
    filter: FilteredDerivative,
    integral_term: f32,
}

impl PidController {
    /// Creates a new PID controller with a filtered derivative term using the provided gain configuration
    pub fn new(config: PidGain) -> Self {
        let tau = 1.0 / config.filter_coefficient;
        let filter = FilteredDerivative::new(config.kd, tau, SAMPLING_PERIOD);
        Self {
            config,
            filter,
            integral_term: 0.0,
        }
    }
}

impl ControllerTrait for PidController {
    /// Updates the PID controller and returns the controller output
    ///
    /// Arguments:
    /// * `set_point` - The target value
    /// * `actual` - The current measured value
    /// * `dt` - The time step since the last update (TODOLater decide units)
    fn update(&mut self, set_point: f32, actual: f32, dt: u64) -> f32 {
        let p_error = (set_point * self.config.p_reference_gain) - actual;
        let i_error = set_point - actual;
        let d_error = (set_point * self.config.d_reference_gain) - actual;

        self.integral_term += i_error * dt as f32;
        let d_filtered = self.filter.update(d_error);

        self.config.kp * p_error + self.config.ki * self.integral_term + self.config.kd * d_filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the PID controller returns 0 when all values are 0
    #[test]
    fn test_pid_controller() {
        let config = PidGain {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
            p_reference_gain: 1.0,
            d_reference_gain: 1.0,
            filter_coefficient: 0.1,
        };
        let mut pid = PidController::new(config);
        let set_point = 0.0;
        let actual = 0.0;
        let dt = 1;
        let output = pid.update(set_point, actual, dt);
        assert_eq!(output, 0.0);
    }
}
