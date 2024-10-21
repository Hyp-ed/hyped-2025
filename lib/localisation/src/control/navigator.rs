
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

    pub fn update(&mut self) {

        let measurement = DVector::zeros(self.kalman_filter.get_state().len());
        
        self.kalman_filter.predict();
        self.kalman_filter.update(measurement);
        self.displacement = self.kalman_filter.get_state()[0];
        self.velocity = self.kalman_filter.get_state()[1];
        self.acceleration = self.kalman_filter.get_state()[2];
    }
        


}