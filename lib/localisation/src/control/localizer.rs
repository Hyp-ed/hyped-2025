use crate::{
    filtering::kalman_filter::KalmanFilter,
    preprocessing::{
        accelerometer::AccelerometerPreprocessor, keyence::{KeyenceAgrees, SensorChecks},
        optical::process_optical_data,
    },
};

use heapless::Vec;
use libm::pow;
use nalgebra::{Matrix2, Vector2};

const DELTA_T: f64 = 0.01;

pub struct Localizer {
    displacement: f64,
    velocity: f64,
    acceleration: f64,
    kalman_filter: KalmanFilter,
    accelerometer_preprocessor: AccelerometerPreprocessor,
    keyence_checker: KeyenceAgrees,
}

impl Localizer {
    pub fn new(kalman_filter: KalmanFilter) -> Localizer {
        let initial_state = Vector2::new(0.0, 0.0);
        let initial_covariance = Matrix2::new(1.0, 0.0, 0.0, 1.0);
        let transition_matrix = Matrix2::new(1.0, DELTA_T, 0.0, 1.0);
        let control_matrix = Vector2::new(0.5 * DELTA_T * DELTA_T, DELTA_T);
        let observation_matrix = Matrix2::new(1.0, 0.0, 0.0, DELTA_T);

        // Assuming frequency of 6400hz for IMU at 120 mu g / sqrt(Hz)
        // standard deviation = 120 * sqrt(6400) = 9600 mu g = 0.0096 g
        //                    = 0.0096 * 9.81 = 0.094176 m/s^2
        // variance =  0.094176^2 = 0.0089 m/s^2

        let process_noise: Matrix2<f64> = Matrix2::new(
            0.25 * pow(DELTA_T, 4.0),
            0.5 * pow(DELTA_T, 3.0),
            0.5 * pow(DELTA_T, 3.0),
            pow(DELTA_T, 2.0) * 0.0089,
        );

        //  We assume the stripe counter is accurate
        //  Optical flow expects standard deviation of 0.01% of the measured value
        //  Assuming top speed 10m/s,
        //  standard deviation = 0.01 * 10 = 0.1 m/s
        //  variance = 0.1^2 = 0.01 m/s^2

        let measurement_noise: Matrix2<f64> = Matrix2::new(0.01, 0.0, 0.0, 0.0);

        let kalman_filter = KalmanFilter::new(
            initial_state,
            initial_covariance,
            transition_matrix,
            control_matrix,
            observation_matrix,
            process_noise,
            measurement_noise,
        );

        Localizer {
            displacement: 0.0,
            velocity: 0.0,
            acceleration: 0.0,
            kalman_filter,
            accelerometer_preprocessor: AccelerometerPreprocessor::new(),
            keyence_checker: KeyenceAgrees::new(),
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

    pub fn preprocessor(
        &mut self,
        optical_data: Vec<Vec<f64, 2>, 2>,
        keyence_data: Vec<bool, 2>,
        accelerometer_data: Vec<f64, 4>,
    ) {

        let processed_optical_data = process_optical_data(optical_data);

        let keyence_status = self.keyence_checker.check_keyence_agrees(keyence_data.clone());
        //if keyence_status == SensorChecks::Unacceptable {
            //TODOLater: Change state
        //    return;
        //}




        
    
    }

}
