pub trait ControllerTrait {
    fn update(&mut self, set_point: f32, actual: f32, dt: u64) -> f32;
}

#[derive(Debug, Clone)]
pub struct PiGain {
    pub kp: f32,
    pub ki: f32,
}

#[derive(Debug, Clone)]
pub struct PiController {
    config: PiGain,
    integral_term: f32,
}

impl PiController {
    pub fn new(config: PiGain) -> Self {
        Self {
            config,
            integral_term: 0.0,
        }
    }
}

impl ControllerTrait for PiController {
    /// Updates the `Pi` controller, ignoring D.
    /// Returns the controller output.
    fn update(&mut self, set_point: f32, actual: f32, dt: u64) -> f32 {
        let error = set_point - actual;
        self.integral_term += error * dt as f32;
        self.config.kp * error + self.config.ki * self.integral_term // removed the derivative term
                                                                     // TODOLater could restrict output by min value here instead of using .min()
    }
}

#[derive(Debug, Clone)]
struct FilteredDerivative {
    b0: f32,
    b1: f32,
    a1: f32,
    previous_error: f32,
    previous_output: f32,
}

impl FilteredDerivative {
    fn new(kd: f32, tau: f32, sampling_period: u64) -> Self {
        let sampling_period = sampling_period as f32;
        let denominator = sampling_period * (tau + sampling_period) + 2.0;

        FilteredDerivative {
            b0: 2.0 * kd / denominator,
            b1: -2.0 * kd / denominator,
            a1: 2.0 * sampling_period / denominator,
            previous_error: 0.0,
            previous_output: 0.0,
        }
    }

    fn update(&mut self, error: f32) -> f32 {
        let output =
            self.b0 * error + self.b1 * self.previous_error - self.a1 * self.previous_output;

        // Update previous values for the next iteration
        self.previous_error = error;
        self.previous_output = output;

        output
    }
}

#[derive(Debug, Clone)]
pub struct PidGain {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub p_reference_gain: f32,
    pub d_reference_gain: f32,
    pub filter_coefficient: f32,
}

#[derive(Debug, Clone)]
pub struct PidController {
    config: PidGain,
    filter: FilteredDerivative,
    integral_term: f32,
}

impl PidController {
    pub fn new(config: PidGain) -> Self {
        let filter = FilteredDerivative::new(config.kd, 1.0 / config.filter_coefficient, 1); // TODOLater sub with SAMPLING PERIOD CONSTANT in .yaml file

        Self {
            config,
            filter,
            integral_term: 0.0,
        }
    }
}

impl ControllerTrait for PidController {
    fn update(&mut self, set_point: f32, actual: f32, dt: u64) -> f32 {
        let p_error = (set_point * self.config.p_reference_gain) - actual;
        let i_error = set_point - actual;
        let d_error = (set_point * self.config.d_reference_gain) - actual;
        self.integral_term += i_error * dt as f32;
        let d_filtered = self.filter.update(d_error);
        self.config.kp * p_error + self.config.ki * self.integral_term + self.config.kd * d_filtered
        // TODOLater Maybe could restrict output by min value here instead of using .min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pi_controller() {
        let config = PiGain { kp: 1.0, ki: 0.0 };
        let mut pi = PiController::new(config);
        let set_point = 0.0;
        let actual = 0.0;
        let dt = 1;
        let output = pi.update(set_point, actual, dt);
        assert_eq!(output, 0.0);
    }

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
