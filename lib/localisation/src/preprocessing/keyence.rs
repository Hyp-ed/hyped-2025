use heapless::Vec;

#[derive(PartialEq, Debug)]
pub enum SensorChecks {
    Acceptable,
    Unnaceptable,
}

/// This struct checks if both the current and previous keyance data is in disagreement
pub struct KeyenceAgrees {
    previous_keyance_agreement: bool,
}

impl KeyenceAgrees {
    pub fn new() -> Self {
        KeyenceAgrees {
            previous_keyance_agreement: true,
        }
    }

    pub fn check_keyence_agrees(&mut self, keyence_data: Vec<bool, 2>) -> SensorChecks {
        if keyence_data[0] != keyence_data[1] && !self.previous_keyance_agreement {
            return SensorChecks::Unnaceptable;
        } else {
            self.previous_keyance_agreement = keyence_data[0] == keyence_data[1];
        }

        SensorChecks::Acceptable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable_success() {
        let keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, true]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let result = keyence_agrees.check_keyence_agrees(keyence_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_acceptable_false_success() {
        let keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, false]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let result = keyence_agrees.check_keyence_agrees(keyence_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_acceptable_second_false_success() {
        let first_keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, true]).unwrap();
        let second_keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, false]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let initial_try = keyence_agrees.check_keyence_agrees(first_keyence_data);
        let result = keyence_agrees.check_keyence_agrees(second_keyence_data);
        assert_eq!(initial_try, desired_outcome);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_acceptable_prev_false_success() {
        let first_keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, false]).unwrap();
        let second_keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, true]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let initial_try = keyence_agrees.check_keyence_agrees(first_keyence_data);
        let result = keyence_agrees.check_keyence_agrees(second_keyence_data);
        assert_eq!(initial_try, desired_outcome);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_unnacceptable_prev_false_success() {
        let first_keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, false]).unwrap();
        let second_keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, false]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let first_outcome = SensorChecks::Acceptable;
        let second_outcome = SensorChecks::Unnaceptable;
        let initial_try = keyence_agrees.check_keyence_agrees(first_keyence_data);
        let result = keyence_agrees.check_keyence_agrees(second_keyence_data);
        assert_eq!(initial_try, first_outcome);
        assert_eq!(result, second_outcome);
    }
}
