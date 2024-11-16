use crate::filtering::kalman_filter::KalmanFilter;



pub struct Localizer {
    displacement: f64,
    velocity: f64,
    acceleration: f64,
    kalman_filter: KalmanFilter,
}

impl Localizer {
    pub fn new(kalman_filter: KalmanFilter) -> Localizer {
        Localizer {
            displacement: 0.0,
            velocity: 0.0,
            acceleration: 0.0,
            kalman_filter,
        }
    }

    //Setters

    pub fn set_displacement(&mut self, displacement: f64) {
        self.displacement = displacement;
    }

    pub fn set_velocity(&mut self, velocity: f64) {
        self.velocity = velocity;
    }

    pub fn set_acceleration(&mut self, acceleration: f64) {
        self.acceleration = acceleration;
    }

    //Getters

    pub fn get_displacement(&self) -> f64 {
        self.displacement
    }

    pub fn get_velocity(&self) -> f64 {
        self.velocity
    }

    pub fn get_acceleration(&self) -> f64 {
        self.acceleration
    }
}
