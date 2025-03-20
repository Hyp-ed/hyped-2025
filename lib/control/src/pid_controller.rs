pub trait PidController {
    fn new(config: PidGain) -> Self;
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct PidGain {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub p_reference_gain: f32,
    pub d_reference_gain: f32,
    pub filter_constant: f32,
}
#[derive(Debug, Clone)]
pub struct Pid {
    pub config: PidGain,
    pub integral_term: f32,
    pub pre_error: f32,
    pub current_filter: f32,
}

impl PidController for Pid {
    fn new(config: PidGain) -> Self {
        Self {
            config,
            integral_term: 0.0,
            pre_error: 0.0,
            current_filter: 0.0,
        }
    }

    /// Updates the `Pid` controller with the specified set point, actual value, and time delta.
    /// Implements a low pass filter onto the derivative term.
    /// Returns the controller output.
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32 {
        let p_error = (set_point * self.config.p_reference_gain) - actual;
        let i_error = set_point - actual;
        let d_error = (set_point * self.config.d_reference_gain) - actual;
        self.integral_term += i_error * dt;
        let unfiltered_derivative = (d_error - self.pre_error) / dt;
        self.current_filter +=
            dt * self.config.filter_constant * (unfiltered_derivative - self.current_filter);
        self.pre_error = d_error;
        self.config.kp * p_error
            + self.config.ki * self.integral_term
            + self.config.kd * self.current_filter
        // TODOLater Maybe could restrict output by min value here instead of using .min()
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
            filter_constant: 0.1,
        };
        let mut pid = Pid::new(config);
        let set_point = 0.0;
        let actual = 0.0;
        let dt = 0.1;
        let output = pid.update(set_point, actual, dt);
        assert_eq!(output, 0.0);
    }
}
