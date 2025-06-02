use crate::config::NUM_KEYENCE_SENSORS;
use heapless::Vec;

#[derive(PartialEq, Debug)]
pub enum SensorChecks {
    Acceptable,
    Unacceptable,
}

/// Checks if the two Keyence sensors are in agreement.
/// If the sensors disagree for two consecutive readings, the check fails.
pub struct KeyenceAgrees {
    previous_keyence_agreement: bool,
}

impl Default for KeyenceAgrees {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyenceAgrees {
    pub fn new() -> Self {
        KeyenceAgrees {
            previous_keyence_agreement: true,
        }
    }

    pub fn check_keyence_agrees(
        &mut self,
        keyence_data: Vec<u32, NUM_KEYENCE_SENSORS>,
    ) -> SensorChecks {
        // TODOLater: support more than 2 sensors
        if keyence_data[0] != keyence_data[1] && !self.previous_keyence_agreement {
            return SensorChecks::Unacceptable;
        } else {
            self.previous_keyence_agreement = keyence_data[0] == keyence_data[1];
        }

        SensorChecks::Acceptable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable_success() {
        let keyence_data: Vec<u32, 2> = Vec::from_slice(&[0, 1]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let result = keyence_agrees.check_keyence_agrees(keyence_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_acceptable_false_success() {
        let keyence_data: Vec<u32, 2> = Vec::from_slice(&[0, 1]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let result = keyence_agrees.check_keyence_agrees(keyence_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_acceptable_second_false_success() {
        let first_keyence_data: Vec<u32, 2> = Vec::from_slice(&[1, 1]).unwrap();
        let second_keyence_data: Vec<u32, 2> = Vec::from_slice(&[1, 1]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let initial_try = keyence_agrees.check_keyence_agrees(first_keyence_data);
        let result = keyence_agrees.check_keyence_agrees(second_keyence_data);
        assert_eq!(initial_try, desired_outcome);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_acceptable_prev_false_success() {
        let first_keyence_data: Vec<u32, 2> = Vec::from_slice(&[1, 2]).unwrap();
        let second_keyence_data: Vec<u32, 2> = Vec::from_slice(&[1, 1]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let desired_outcome = SensorChecks::Acceptable;
        let initial_try = keyence_agrees.check_keyence_agrees(first_keyence_data);
        let result = keyence_agrees.check_keyence_agrees(second_keyence_data);
        assert_eq!(initial_try, desired_outcome);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_unnacceptable_prev_false_success() {
        let first_keyence_data: Vec<u32, 2> = Vec::from_slice(&[1, 2]).unwrap();
        let second_keyence_data: Vec<u32, 2> = Vec::from_slice(&[2, 3]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new();
        let first_outcome = SensorChecks::Acceptable;
        let second_outcome = SensorChecks::Unacceptable;
        let initial_try = keyence_agrees.check_keyence_agrees(first_keyence_data);
        let result = keyence_agrees.check_keyence_agrees(second_keyence_data);
        assert_eq!(initial_try, first_outcome);
        assert_eq!(result, second_outcome);
    }
}
