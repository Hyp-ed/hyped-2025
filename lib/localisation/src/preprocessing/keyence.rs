use heapless::Vec;

#[derive(PartialEq, Debug)]
pub enum SensorChecks {
    Acceptable,
    Unnaceptable,
}

pub struct KeyenceAgrees {
    keyence_data: Vec<bool, 2>,
    previous_keyance_agreement: bool,
}

impl KeyenceAgrees {
    pub fn new(keyence_data: Vec<bool, 2>) -> Self {
        KeyenceAgrees {
            keyence_data,
            previous_keyance_agreement: true,
        }
    }

    pub fn check_keyence_agrees(&mut self) -> SensorChecks {
        if self.keyence_data[0] != self.keyence_data[1] && !self.previous_keyance_agreement {
            return SensorChecks::Unnaceptable;
        } else if self.keyence_data[0] != self.keyence_data[1] {
            self.previous_keyance_agreement = false;
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
        let mut keyence_agrees = KeyenceAgrees::new(keyence_data);
        let desired_outcome = SensorChecks::Acceptable;
        let result = keyence_agrees.check_keyence_agrees();
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_uncceptable_success() {
        let keyence_data: Vec<bool, 2> = Vec::from_slice(&[true, true]).unwrap();
        let mut keyence_agrees = KeyenceAgrees::new(keyence_data);
        let desired_outcome = SensorChecks::Acceptable;
        let result = keyence_agrees.check_keyence_agrees();
        assert_eq!(result, desired_outcome);
    }
}
