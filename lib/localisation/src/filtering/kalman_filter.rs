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

    // Predict step
    pub fn predict(&mut self, control_input: &DVector<f64>) {
        // x_k = F * x_k-1 + B * u_k
        self.state = &self.transition_matrix * &self.state + &self.control_matrix * control_input;

        // P_k = F * P_k-1 * F^T + Q
        self.covariance =
            &self.transition_matrix * &self.covariance * self.transition_matrix.transpose()
                + &self.process_noise;
    }

    // Update step
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
    use super::KalmanFilter;
    use nalgebra::{DMatrix, DVector};

    #[test]

    fn test_kalman_filter() {
        let initial_state = DVector::from_column_slice(&[0.0, 0.0]);
        let initial_covariance = DMatrix::from_diagonal_element(2, 2, 500.0);

        let transition_matrix = DMatrix::from_row_slice(2, 2, &[1.0, 0.25, 0.0, 1.0]);
        let control_matrix = DMatrix::from_row_slice(2, 1, &[0.0313, 0.25]);
        let observation_matrix = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);

        let process_noise =
            DMatrix::from_row_slice(2, 2, &[0.00000976562, 0.000078125, 0.000078125, 0.0625]);
        let measurement_noise = DMatrix::from_diagonal_element(1, 1, 400.0);

        let mut kalman_filter = KalmanFilter::new(
            initial_state,
            initial_covariance,
            transition_matrix,
            control_matrix,
            observation_matrix,
            process_noise,
            measurement_noise,
        );

        let control_input = DVector::from_column_slice(&[0.0]);
        kalman_filter.predict(&control_input);

        let state = kalman_filter.get_state();
        assert!((state - DVector::from_column_slice(&[0.0, 0.0])).norm() < 1e-2);

        let h_values = DVector::from_column_slice(&[
            6.43, 1.3, 39.43, 45.89, 41.44, 48.7, 78.06, 80.08, 61.77, 75.15, 110.39, 127.83,
            158.75, 156.55, 213.32, 229.82, 262.8, 297.57, 335.69, 367.92, 377.19, 411.18, 460.7,
            468.39, 553.9, 583.97, 655.15, 723.09, 736.85, 787.22,
        ]);

        let a_values = DVector::from_column_slice(&[
            39.81, 39.67, 39.81, 39.84, 40.05, 39.85, 39.78, 39.65, 39.67, 39.78, 39.59, 39.87,
            39.85, 39.59, 39.84, 39.9, 39.63, 39.59, 39.76, 39.79, 39.73, 39.93, 39.83, 39.85,
            39.94, 39.86, 39.76, 39.86, 39.74, 39.94,
        ]);

        for i in 0..h_values.len() {
            let measurement = DVector::from_column_slice(&[h_values[i]]);
            kalman_filter.update(&measurement);

            let control_input = DVector::from_column_slice(&[a_values[i] - 9.8]);
            kalman_filter.predict(&control_input);
        }

        let final_state = kalman_filter.get_state();
        assert!((final_state - DVector::from_column_slice(&[851.9, 223.2])).norm() < 0.5);
    }
}
