use nalgebra::{Matrix2, Vector1, Vector2};

pub struct KalmanFilter {
    // Current state estimate (2x1)
    state: Vector2<f64>,

    // Current error covariance (2x2)
    covariance: Matrix2<f64>,

    // State transition matrix (2x2)
    transition_matrix: Matrix2<f64>,

    // Control matrix (2x1)
    control_matrix: Vector2<f64>,

    // Observation matrix (1x2)
    observation_matrix: Matrix2<f64>,

    // Process noise covariance (2x2)
    process_noise: Matrix2<f64>,

    // Measurement noise covariance (1x1)
    measurement_noise: Matrix2<f64>,
}

impl KalmanFilter {
    pub fn new(
        initial_state: Vector2<f64>,
        initial_covariance: Matrix2<f64>,
        transition_matrix: Matrix2<f64>,
        control_matrix: Vector2<f64>,
        observation_matrix: Matrix2<f64>,
        process_noise: Matrix2<f64>,
        measurement_noise: Matrix2<f64>,
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

    /// Predict step
    /// Predicts the next state of the system, uses the accelerometer data.
    pub fn predict(&mut self, control_input: &Vector1<f64>) {
        // x_k = F * x_k-1 + B * u_k
        self.state = &self.transition_matrix * &self.state + &self.control_matrix * control_input;

        // P_k = F * P_k-1 * F^T + Q
        self.covariance =
            &self.transition_matrix * &self.covariance * self.transition_matrix.transpose()
                + &self.process_noise;
    }

    /// Update step: Corrects the state estimate based on the measurement. Uses the stripe counter and optical flow data.
    pub fn update(&mut self, measurement: &Vector2<f64>) {
        // y_k = z_k - H * x_k
        let innovation = measurement - &self.observation_matrix * &self.state;

        // S = H * P_k * H^T + R
        let innovation_covariance =
            &self.observation_matrix * &self.covariance * self.observation_matrix.transpose()
                + &self.measurement_noise;

        let a = innovation_covariance[(0, 0)];
        let b = innovation_covariance[(0, 1)];
        let c = innovation_covariance[(1, 0)];
        let d = innovation_covariance[(1, 1)];

        let determinant = a * d - b * c;

        let innovation_covariance_inv = Matrix2::new(d, -b, -c, a) / determinant;

        // K = P_k * H^T * S^-1
        let kalman_gain =
            &self.covariance * self.observation_matrix.transpose() * innovation_covariance_inv;

        self.state = &self.state + &kalman_gain * innovation;

        let identity = Matrix2::identity();
        self.covariance = (identity - &kalman_gain * &self.observation_matrix) * &self.covariance;
    }

    pub fn get_state(&self) -> Vector2<f64> {
        self.state.clone()
    }
}

/*
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
*/
