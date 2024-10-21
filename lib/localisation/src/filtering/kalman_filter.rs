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


//DEFINE THE MATRICES 

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
        self.covariance =
            &self.transition_matrix * &self.covariance * self.transition_matrix.transpose()
                + &self.process_noise;
    }

    // Update
    pub fn update(&mut self, measurement: DVector<f64>) {
        // K_k = P_k*H^T*(H*P_k*H^T + R)^-1
        let kalman_gain = &self.covariance
            * self.observation_matrix.transpose()
            * (&self.observation_matrix * &self.covariance * self.observation_matrix.transpose()
                + &self.measurement_noise)
                .try_inverse()
                .unwrap();

        // x_k = x_k + K_k*(z_k - H*x_k)
        self.state =
            &self.state + &kalman_gain * (measurement - &self.observation_matrix * &self.state);

        // P_k = (I - K_k*H)*P_k*(I - K_k*H)^T + K_k*R*K_k^T
        let identity = DMatrix::<f64>::identity(self.covariance.nrows(), self.covariance.ncols());
        let difference = &identity - &kalman_gain * &self.observation_matrix;
        self.covariance = &difference * &self.covariance * difference.transpose()
            + &kalman_gain * &self.measurement_noise * kalman_gain.transpose();
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
        
        let initial_state = DVector::zeros(3);
        let initial_covariance = DMatrix::identity(3, 3);
        let transition_matrix = DMatrix::identity(3, 3);
        let observation_matrix = DMatrix::identity(3, 3);
        let process_noise = DMatrix::identity(3, 3);
        let measurement_noise = DMatrix::identity(3, 3);
        

        // Create the Kalman filter
        let mut kalman_filter = KalmanFilter::new(
            initial_state,
            initial_covariance,
            transition_matrix,
            observation_matrix,
            process_noise,
            measurement_noise,
        );

        // Predict
        kalman_filter.predict();

        // Update
        let measurement = DVector::zeros(3);
        kalman_filter.update(measurement);

        // Get the state
        let state = kalman_filter.get_state();

        // Check the state
        println!("{:?}", state);
        assert_eq!(state, DVector::zeros(3));
    }

}
