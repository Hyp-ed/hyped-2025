pub trait PidController {
    fn new(config: PidGain) -> Self;
    fn update(&mut self, set_point: f32, actual: f32, dt: f32, filter_constant: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct PidGain {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub p_reference_gain: f32,
    pub d_reference_gain: f32,
}
/// `Pid` is a structure that implements the [`PidController`] trait.
#[derive(Debug, Clone)]
pub struct Pid {
    pub config: PidGain,
    pub i_term: f32,
    pub pre_error: f32,
    pub current_filter: f32,
}

impl PidController for Pid {
    /// Updates the `Pid` controller with the specified set point, actual value, and time delta.
    /// Implements a low pass filter onto the derivative term.
    /// Returns the controller output.
    fn new(config: PidGain) -> Self {
        Self {
            config,
            i_term: 0.0,
            pre_error: 0.0,
            current_filter: 0.0,
        }
    }
    fn update(&mut self, set_point: f32, actual: f32, dt: f32, filter_constant: f32) -> f32 {
        let p_error = (set_point * self.config.p_reference_gain) - actual;
        let i_error = set_point - actual;
        let d_error = (set_point * self.config.d_reference_gain) - actual;
        self.i_term += i_error * dt;
        let unfiltered_derivative = (d_error - self.pre_error) / dt;
        self.current_filter += dt * filter_constant * (unfiltered_derivative - self.current_filter);
        self.pre_error = d_error;
        let output = self.config.kp * p_error
            + self.config.ki * self.i_term
            + self.config.kd * self.current_filter;

        output // TOMaybeDO could restrict output by min value here instead of using .min()
    }
}

pub trait PiController {
    fn new(config: PiGain) -> Self;
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct PiGain {
    pub kp: f32,
    pub ki: f32,
}

#[derive(Debug, Clone)]
pub struct Pi {
    pub config: PiGain,
    pub i_term: f32,
    pub pre_error: f32,
}

impl PiController for Pi {
    /// Creates a new `Pi` with the specified configuration.
    fn new(config: PiGain) -> Self {
        Self {
            config,
            i_term: 0.0,
            pre_error: f32::NAN,
        }
    }
    /// Updates the `Pi` controller, ignoring D.
    /// Returns the controller output.
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32 {
        let error = set_point - actual;
        self.i_term += error * dt;
        let output = self.config.kp * error + self.config.ki * self.i_term; // removed the derivative term
        self.pre_error = error;
        output // TOMaybeDO could restrict output by min value here instead of using .min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_controller() {
        let config = PidGain {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
            p_reference_gain: 1.0,
            d_reference_gain: 1.0,
        };
        let mut pid = Pid::new(config);
        let set_point = 0.0;
        let actual = 0.0;
        let dt = 0.1;
        let filter_constant = 0.1;
        let output = pid.update(set_point, actual, dt, filter_constant);
        assert_eq!(output, 0.0);
    }

    #[test]
    fn test_pi_controller() {
        let config = PiGain { kp: 1.0, ki: 0.0 };
        let mut pi = Pi::new(config);
        let set_point = 0.0;
        let actual = 0.0;
        let dt = 0.1;
        let output = pi.update(set_point, actual, dt);
        assert_eq!(output, 0.0);
    }
}
