use crate::config::NUM_OPTICAL_FLOW_SENSORS;
use heapless::Vec;
use libm::sqrtf;

/// Processes the raw optical data to get the magnitude and added to the optical data for each sensor
pub fn process_optical_data(raw_optical_data: Vec<Vec<f64, 2>, NUM_OPTICAL_FLOW_SENSORS>) -> f32 {
    let mut total_magnitude: f32 = 0.0;

    for sensor_data in raw_optical_data {
        let mut magnitude: f32 = 0.0;

        for data in sensor_data {
            let data: f32 = data as f32;
            magnitude += data * data;
        }
        total_magnitude += sqrtf(magnitude);
    }

    total_magnitude / NUM_OPTICAL_FLOW_SENSORS as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::NUM_OPTICAL_FLOW_SENSORS;

    #[test]
    fn test_correct_positive() {
        let raw_optical_data: Vec<Vec<f64, 2>, NUM_OPTICAL_FLOW_SENSORS> =
            Vec::from_slice(&[Vec::from_slice(&[1.0, 1.0]).unwrap()]).unwrap();
        let desired_outcome: f32 = sqrtf(2.0);
        let result = process_optical_data(raw_optical_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_correct_negative() {
        let raw_optical_data: Vec<Vec<f64, 2>, NUM_OPTICAL_FLOW_SENSORS> =
            Vec::from_slice(&[Vec::from_slice(&[-4.0, -6.0]).unwrap()]).unwrap();
        let desired_outcome: f32 = sqrtf(52.0);
        let result = process_optical_data(raw_optical_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_correct_zero() {
        let raw_optical_data: Vec<Vec<f64, 2>, NUM_OPTICAL_FLOW_SENSORS> =
            Vec::from_slice(&[Vec::from_slice(&[0.0, 0.0]).unwrap()]).unwrap();
        let desired_outcome: f32 = 0.0;
        let result = process_optical_data(raw_optical_data);
        assert_eq!(result, desired_outcome);
    }
}
