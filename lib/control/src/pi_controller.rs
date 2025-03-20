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
    config: PiGain,
    i_term: f32,
    pre_error: f32,
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
