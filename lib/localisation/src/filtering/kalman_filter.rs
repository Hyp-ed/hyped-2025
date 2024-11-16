use nalgebra::{Matrix2, Vector1, Vector2};

/// Kalman filter implementation for cart for sensor fusion.
/// Recursively estimates the state from a series of nois measurements.
///
/// Uses keyence, optical flow and accelerometer data.
///
/// Control input: Accelerometer data
/// Measurement: Optical flow data, keyence stripe counter
pub struct KalmanFilter {
    /// Current state estimate (2x1)
    state: Vector2<f64>,
    /// Current error covariance (2x2)
    covariance: Matrix2<f64>,
    /// State transition matrix (2x2)
    transition_matrix: Matrix2<f64>,
    /// Control matrix (2x1)
    control_matrix: Vector2<f64>,
    /// Observation matrix (1x2)
    observation_matrix: Matrix2<f64>,
    /// Process noise covariance (2x2)
    process_noise: Matrix2<f64>,
    /// Measurement noise covariance (1x1)
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
    /// Predicts the next state of the system, using the accelerometer data as control input.
    /// Predicts the next state covariance.
    pub fn predict(&mut self, control_input: &Vector1<f64>) {
        // x_k = F * x_k-1 + B * u_k
        self.state = self.transition_matrix * self.state + self.control_matrix * control_input;

        // P_k = F * P_k-1 * F^T + Q
        self.covariance =
            self.transition_matrix * self.covariance * self.transition_matrix.transpose()
                + self.process_noise;
    }

    /// Update step: Corrects the state estimate based on the measurement.
    /// Uses the stripe counter (displacement) and optical flow data (velocity).
    pub fn update(&mut self, measurement: &Vector2<f64>) {
        // y_k = z_k - H * x_k
        let innovation = measurement - self.observation_matrix * self.state;

        // S = H * P_k * H^T + R
        let innovation_covariance =
            self.observation_matrix * self.covariance * self.observation_matrix.transpose()
                + self.measurement_noise;

        // Calculate the inverse of the innovation covariance
        // The innovationn covariance is always full rank, so the inverse always exists
        let a = innovation_covariance[0];
        let b = innovation_covariance[1];
        let c = innovation_covariance[2];
        let d = innovation_covariance[3];

        let det = a * d - b * c
        let innovation_covariance_inv = Matrix2::new(d, -b, -c, a) / det;
        
        // K = P_k * H^T * S^-1
        let kalman_gain =
            self.covariance * self.observation_matrix.transpose() * innovation_covariance_inv;

        self.state += kalman_gain * innovation;

        let identity = Matrix2::identity();
        self.covariance = (identity - kalman_gain * self.observation_matrix) * self.covariance;
    }

    pub fn get_state(&self) -> Vector2<f64> {
        self.state
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use nalgebra::{Matrix2, Vector1, Vector2};

    #[test]
    fn test_kalman_filter() {
        // Test simulating simple cart movement

        // Acceleration: 20ms^-2
        // Initial velocity: 0
        // Initial position: 0
        // Measurements taken every 1s
        // 20 measurements taken

        // Variance of measurements:
        //  Distance: 10
        //  Velocity: 5
        //  Acceleration: 3

        // Expected displacement: 4000m
        // Expected velocity: 400ms^-1

        let initial_state = Vector2::new(0.0, 0.0);
        let initial_covariance = Matrix2::new(1.0, 0.0, 0.0, 1.0);
        let transition_matrix = Matrix2::new(1.0, 1.0, 0.0, 1.0);
        let control_matrix = Vector2::new(0.5, 1.0);
        let process_noise = Matrix2::new(0.25 * 3.0, 0.5 * 3.0, 0.5 * 3.0, 1.0 * 3.0);
        let observation_matrix = Matrix2::new(1.0, 0.0, 0.0, 1.0);
        let measurement_noise = Matrix2::new(1.0, 0.0, 0.0, 0.0);

        let mut kalman_filter = KalmanFilter::new(
            initial_state,
            initial_covariance,
            transition_matrix,
            control_matrix,
            observation_matrix,
            process_noise,
            measurement_noise,
        );

        let acc_measurements = [
            20.38, 15.02, 19.17, 19.36, 20.6, 16.89, 20.42, 20.33, 21.53, 19.98, 26.08, 17.63,
            21.01, 17.06, 21.83, 23.18, 26.03, 17.67, 22.99, 18.78,
        ];
        let dist_measurements = [
            28.59, 51.7, 87.64, 169.39, 244.96, 363.2, 493.0, 626.97, 817.84, 1021.23, 1192.21,
            1423.34, 1689.46, 1964.72, 2261.09, 2565.47, 2902.0, 3239.25, 3627.35, 4011.47,
        ];
        let vel_measurements = [
            20.06, 42.43, 66.23, 83.03, 104.47, 122.93, 135.85, 157.55, 179.02, 201.75, 216.53,
            240.63, 262.97, 285.75, 307.43, 321.04, 331.48, 354.47, 374.19, 394.29,
        ];

        for i in 0..20 {
            let measurement = Vector2::new(dist_measurements[i], vel_measurements[i]);
            let control_input = Vector1::new(acc_measurements[i]);

            kalman_filter.predict(&control_input);
            kalman_filter.update(&measurement);
        }

        let final_state = kalman_filter.get_state();
        assert!(final_state[0] - 4000.0 < 1e-10);
        assert!(final_state[1] - 400.0 < 1e-10);
    }
}
