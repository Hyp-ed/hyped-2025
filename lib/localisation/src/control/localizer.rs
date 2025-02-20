use crate::{
    filtering::kalman_filter::KalmanFilter,
    preprocessing::{
        accelerometer::AccelerometerPreprocessor,
        keyence::{KeyenceAgrees, SensorChecks},
        optical::process_optical_data,
    },
    types::{RawAccelerometerData, NUM_ACCELEROMETERS, NUM_AXIS},
};

use heapless::Vec;
use libm::pow;
use nalgebra::{Matrix2, Vector1, Vector2};

//TODOLater: Confirm values are correct

// Time step (s)
const DELTA_T: f64 = 0.01;

// Stripe width (m)
const STRIPE_WIDTH: f64 = 1.0;

pub struct Localizer {
    displacement: f64,
    velocity: f64,
    previous_velocity: f64,
    acceleration: f64,
    kalman_filter: KalmanFilter,
    keyence_checker: KeyenceAgrees,
    keyence_val: f64,
    optical_val: f64,
    accelerometer_val: f64,
    accelerometer_preprocessor: AccelerometerPreprocessor,
}

impl Localizer {
    pub fn new() -> Localizer {
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
            previous_velocity: 0.0,
            acceleration: 0.0,
            kalman_filter,
            keyence_checker: KeyenceAgrees::new(),
            keyence_val: 0.0,
            optical_val: 0.0,
            accelerometer_val: 0.0,
            accelerometer_preprocessor: AccelerometerPreprocessor::new(),
        }
    }
}

impl Default for Localizer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum PreprocessorError {
    KeyenceUnacceptable,
    AccelerometerUnnaceptable,
}

impl Localizer {
    /// Preprocesses the data from the sensors, checking for errors and outliers
    pub fn preprocessor(
        &mut self,
        optical_data: Vec<f64, 2>,
        keyence_data: Vec<u32, 2>,
        accelerometer_data: RawAccelerometerData<NUM_ACCELEROMETERS, NUM_AXIS>,
    ) -> Result<(), PreprocessorError> {
        let processed_optical_data =
            process_optical_data(Vec::from_slice(&[optical_data.clone()]).unwrap());
        self.optical_val = processed_optical_data as f64;

        let keyence_status = self
            .keyence_checker
            .check_keyence_agrees(keyence_data.clone());

        if keyence_status == SensorChecks::Unacceptable {
            return Err(PreprocessorError::KeyenceUnacceptable);
        } else {
            self.keyence_val = (keyence_data[0] as f64) * STRIPE_WIDTH;
        }

        let processed_accelerometer_data = self
            .accelerometer_preprocessor
            .process_data(accelerometer_data);
        if processed_accelerometer_data.is_none() {
            return Err(PreprocessorError::AccelerometerUnnaceptable);
        }

        let processed_accelerometer_data = processed_accelerometer_data.unwrap();
        self.accelerometer_val = 0.0;
        for i in 0..NUM_ACCELEROMETERS {
            for _ in 0..NUM_AXIS {
                self.accelerometer_val += processed_accelerometer_data[i] as f64;
            }
        }
        self.accelerometer_val /= (NUM_ACCELEROMETERS * NUM_AXIS) as f64;

        Ok(())
    }

    /// Preprocesses the sensor data by calling the preprocessor and then runs the kalman filter
    pub fn iteration(
        &mut self,
        optical_data: Vec<f64, 2>,
        keyence_data: Vec<u32, 2>,
        accelerometer_data: RawAccelerometerData<NUM_ACCELEROMETERS, NUM_AXIS>,
    ) -> Result<(), PreprocessorError> {
        self.preprocessor(
            optical_data.clone(),
            keyence_data.clone(),
            accelerometer_data.clone(),
        )?;

        let control_input = Vector1::new(self.accelerometer_val);

        self.kalman_filter.predict(&control_input);

        let measurement = Vector2::new(self.keyence_val * STRIPE_WIDTH, self.optical_val);

        self.kalman_filter.update(&measurement);

        let state = self.kalman_filter.get_state();

        self.displacement = state[0];
        self.velocity = state[1];
        self.acceleration = self.accelerometer_val;
        self.previous_velocity = self.velocity;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localizer_with_zeros() -> Result<(), PreprocessorError> {
        let mut localizer = Localizer::default();

        let optical_data: Vec<f64, 2> = Vec::from_slice(&[0.0, 0.0]).unwrap();
        let raw_keyence_data: Vec<u32, 2> = Vec::from_slice(&[0, 0]).unwrap();
        let raw_accelerometer_data: RawAccelerometerData<NUM_ACCELEROMETERS, NUM_AXIS> =
            RawAccelerometerData::from_slice(&[
                Vec::from_slice(&[0.0, 0.0, 0.0]).unwrap(),
                Vec::from_slice(&[0.0, 0.0, 0.0]).unwrap(),
                Vec::from_slice(&[0.0, 0.0, 0.0]).unwrap(),
                Vec::from_slice(&[0.0, 0.0, 0.0]).unwrap(),
            ])
            .unwrap();

        localizer.iteration(optical_data, raw_keyence_data, raw_accelerometer_data)?;

        assert_eq!(localizer.displacement, 0.0);
        assert_eq!(localizer.velocity, 0.0);
        assert_eq!(localizer.acceleration, 0.0);

        Ok(())
    }
}
