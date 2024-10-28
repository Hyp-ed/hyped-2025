use nalgebra::{DMatrix, DVector};

pub struct KalmanFilter {
    // Current state estimate
    state: DVector<f64>,

    // Current error covariance
    covariance: DMatrix<f64>,

    // State transition matrix
    transition_matrix: DMatrix<f64>,

    // Control matrix
    control_matrix: DMatrix<f64>,

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
        control_matrix: DMatrix<f64>,
        observation_matrix: DMatrix<f64>,
        process_noise: DMatrix<f64>,
        measurement_noise: DMatrix<f64>,
    ) -> Self {
        KalmanFilter {
            state: initial_state,
            covariance: initial_covariance,
            transition_matrix,
            observation_matrix,
            control_matrix,
            process_noise,
            measurement_noise,
        }
    }

    // Predict
    pub fn predict(&mut self, control_input: &DVector<f64>) {
        // x_k = F * x_k-1 + B * u_k
        self.state = &self.transition_matrix * &self.state + &self.control_matrix * control_input;

        // P_k = F * P_k-1 * F^T + Q
        self.covariance =
            &self.transition_matrix * &self.covariance * self.transition_matrix.transpose()
                + &self.process_noise;
    }

    // Update
    pub fn update(&mut self, measurement: &DVector<f64>) {
        // y_k = z_k - H * x_k
        let innovation = measurement - &self.observation_matrix * &self.state;

        // S = H * P_k * H^T + R
        let innovation_covariance =
            &self.observation_matrix * &self.covariance * self.observation_matrix.transpose()
                + &self.measurement_noise;

        // K = P_k * H^T * S^-1
        let kalman_gain = &self.covariance
            * self.observation_matrix.transpose()
            * innovation_covariance
                .try_inverse()
                .expect("Failed to invert innovation covariance matrix");

        self.state = &self.state + &kalman_gain * innovation;

        let identity = DMatrix::<f64>::identity(self.state.nrows(), self.state.nrows());
        self.covariance = (&identity - &kalman_gain * &self.observation_matrix) * &self.covariance;
    }

    pub fn get_state(&self) -> DVector<f64> {
        self.state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};

    #[test]

    fn test_kalman_filter() {}
}
