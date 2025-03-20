pub trait ControllerTrait {
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32;
}

#[derive(Debug, Clone)]
pub struct PiGain {
    pub kp: f32,
    pub ki: f32,
}

#[derive(Debug, Clone)]
pub struct PiController {
    config: PiGain,
    i_term: f32,
}

impl PiController {
    pub fn new(config: PiGain) -> Self {
        Self {
            config,
            i_term: 0.0,
        }
    }
}

impl ControllerTrait for PiController {
    /// Updates the `Pi` controller, ignoring D.
    /// Returns the controller output.
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32 {
       
    }
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
pub struct PidController {
    config: PidGain,
    i_term: f32,
    pre_error: f32,
    filtered_d: f32,
}


impl PidController {
    pub fn new(config: PidGain) -> Self {
        Self {
            config,
            i_term: 0.0,
            pre_error: 0.0,
            filtered_d: 0.0,
        }
    }
}

impl ControllerTrait for PidController {
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32 {
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
        let dt = 0.1;
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
            filter_constant: 0.1,
        };
        let mut pid = PidController::new(config);
        let set_point = 0.0;
        let actual = 0.0;
        let dt = 0.1;
        let output = pid.update(set_point, actual, dt);
        assert_eq!(output, 0.0);
    }
}
