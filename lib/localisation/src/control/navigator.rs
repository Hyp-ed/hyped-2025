use crate::filtering::kalman_filter::KalmanFilter;
use nalgebra::DVector;

pub struct Navigator {
    displacement: f64,
    velocity: f64,
    acceleration: f64,
    kalman_filter: KalmanFilter,
}

impl Navigator {
    pub fn new(kalman_filter: KalmanFilter) -> Navigator {
        Navigator {
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

    pub fn run(&mut self) {}

    pub fn update(&mut self) {

        let control_input = DVector::from_column_slice(&[0.0]);
        self.kalman_filter.predict(&control_input);

        let measurement = DVector::from_column_slice(&[0.0,0.0]);
        self.kalman_filter.update(&measurement);

        self.set_displacement(self.kalman_filter.get_state()[0]);
        self.set_velocity(self.kalman_filter.get_state()[1]);
        self.set_acceleration(self.kalman_filter.get_state()[2]);
    }
}
