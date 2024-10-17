use crate::preprocessing::keyence::*;
use crate::preprocessing::optical::*;
use crate::preprocessing::accelerometer::*;


pub struct Navigator {

    displacement: f64,
    velocity: f64,
    acceleration: f64,

}

impl Navigator {

    pub fn new() -> Navigator {
        Navigator {
            displacement: 0.0,
            velocity: 0.0,
            acceleration: 0.0,
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