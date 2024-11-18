use crate::{
    filtering::kalman_filter::KalmanFilter,
    preprocessing::{
        accelerometer::AccelerometerPreprocessor,
        keyence::{KeyenceAgrees, SensorChecks},
        optical::process_optical_data,
    },
    types::{RawAccelerometerData, K_NUM_ACCELEROMETERS, K_NUM_AXIS},
};

use heapless::Vec;

use libm::pow;
use nalgebra::{Matrix2, Vector2};

const DELTA_T: f64 = 0.01;
const STRIPE_WIDTH: f64 = 1;

pub struct Localizer {
    displacement: f64,
    velocity: f64,
    acceleration: f64,
    kalman_filter: KalmanFilter,
    accelerometer_preprocessor: AccelerometerPreprocessor,
    keyence_checker: KeyenceAgrees,
    keyence_val: f64,
    optical_val: f64,
    accelerometer_val: f64,

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
            keyence_val: 0.0,
            optical_val: 0.0,
            accelerometer_val: 0.0,
        }
    }

    pub fn preprocessor(
        &mut self,
        optical_data: Vec<Vec<f64, 2>, 2>,
        keyence_data: Vec<u32, 2>,
        accelerometer_data: RawAccelerometerData<K_NUM_ACCELEROMETERS, K_NUM_AXIS>,
    ) {
        let processed_optical_data = process_optical_data(optical_data);

        let keyence_status = self
            .keyence_checker
            .check_keyence_agrees(keyence_data.clone());

        if keyence_status == SensorChecks::Unacceptable {
            //TODOLater: Change state
            return;
        }
        else {
            self.keyence_val = keyence_data[0] as f64;
        }

        

        let mut accelerometer_preprocessor = AccelerometerPreprocessor::new();
        let processed_accelerometer_data =
            accelerometer_preprocessor.process_data(accelerometer_data);
        if processed_accelerometer_data.is_none() {
            // TODOLater: Change state if accelerometer data is unacceptable
            return;
        }

        let processed_accelerometer_data = processed_accelerometer_data.unwrap();
    }

    pub fn update(&mut self) {


        // TODOLater: Implement filtrer + update function
    }
}
