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

impl KalmanFilter {

    pub fn new(
        initial_state: DVector<f64>,
        initial_covariance: DMatrix<f64>,
        transition_matrix: DMatrix<f64>,
        observation_matrix: DMatrix<f64>,
        process_noise: DMatrix<f64>,
        measurement_noise: DMatrix<f64>,
    ) -> Self {
        KalmanFilter {
            state: initial_state,
            covariance: initial_covariance,
            transition_matrix,
            observation_matrix,
            process_noise,
            measurement_noise,
        }
    }

    // Predict
    pub fn predict(&mut self) {

        // x_k = F * x_k-1 + B * u_k
        self.state = &self.transition_matrix * &self.state;

        // P_k = F * P_k-1 * F^T + Q
        self.covariance = &self.transition_matrix * &self.covariance * self.transition_matrix.transpose() + &self.process_noise;
    }

    // Update
    pub fn update(&mut self, measurement: DVector<f64>) {

        // K_k = P_k*H^T*(H*P_k*H^T + R)^-1
        let kalman_gain = &self.covariance * self.observation_matrix.transpose() * (&self.observation_matrix * &self.covariance * self.observation_matrix.transpose() + &self.measurement_noise).try_inverse().unwrap();

        

    
    // x_k = x_k + K_k*(z_k - H*x_k)
    // P_k = (I - K_k*H)*P_k*(I - K_k*H)^T + K_k*R*K_k^T
    }
}  