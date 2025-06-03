use super::ControllerTrait;

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
    /// Updates the `Pi` controller and returns the controller output
    fn update(&mut self, set_point: f32, actual: f32, dt: u64) -> f32 {
        let error = set_point - actual;
        self.integral_term += error * dt as f32;
        self.config.kp * error + self.config.ki * self.integral_term // removed the derivative term
                                                                     // TODOLater could restrict output by min value here instead of using .min()
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
}
