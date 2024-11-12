use heapless::Vec;
use libm::sqrtf;

/// Processes the raw optical data to get the magnitude of the optical data for each sensor
pub fn process_data(raw_optical_data: Vec<Vec<f64, 2>, 2>) -> Vec<f32, 2> {
    let mut optical_data: Vec<f32, 2> = Vec::from_slice(&[0.0, 0.0]).unwrap();

    for i in 0..2 {
        let mut magnitude: f32 = 0.0;

        for data in raw_optical_data[i].clone() {
            let data: f32 = data as f32;
            magnitude += data * data;
        }
        optical_data[i] = sqrtf(magnitude);
    }

    optical_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_positive() {
        let raw_optical_data: Vec<Vec<f64, 2>, 2> = Vec::from_slice(&[
            Vec::from_slice(&[1.0, 1.0]).unwrap(),
            Vec::from_slice(&[3.0, 4.0]).unwrap(),
        ])
        .unwrap();
        let desired_outcome: Vec<f32, 2> = Vec::from_slice(&[1.4142135, 5.0]).unwrap();
        let result = process_data(raw_optical_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_correct_negative() {
        let raw_optical_data: Vec<Vec<f64, 2>, 2> = Vec::from_slice(&[
            Vec::from_slice(&[-4.0, -6.0]).unwrap(),
            Vec::from_slice(&[-3.0, -1.0]).unwrap(),
        ])
        .unwrap();
        let desired_outcome: Vec<f32, 2> = Vec::from_slice(&[7.2111025, 3.1622777]).unwrap();
        let result = process_data(raw_optical_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_correct_zero() {
        let raw_optical_data: Vec<Vec<f64, 2>, 2> = Vec::from_slice(&[
            Vec::from_slice(&[0.0, 0.0]).unwrap(),
            Vec::from_slice(&[0.0, 0.0]).unwrap(),
        ])
        .unwrap();
        let desired_outcome: Vec<f32, 2> = Vec::from_slice(&[0.0, 0.0]).unwrap();
        let result = process_data(raw_optical_data);
        assert_eq!(result, desired_outcome);
    }
}
