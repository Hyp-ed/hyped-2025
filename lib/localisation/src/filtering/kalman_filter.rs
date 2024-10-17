use nalgebra::{DMatrix, DVector};

pub struct KalmanFilter {

    // Current state estimate
    state: DVector<f64>,

    // Current error covariance
    covariance: DMatrix<f64>,

    // State transition matrix
    transition_matrix: DMatrix<f64>,

    // Observation matrix
    observation_matrix: DMatrix<f64>,

    // Process noise covariance
    process_noise: DMatrix<f64>,

    // Measurement noise covariance
    measurement_noise: DMatrix<f64>,

}