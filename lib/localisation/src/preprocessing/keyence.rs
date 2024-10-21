#[derive(PartialEq)]
enum KeyenceDataStatus{
    Agreed,
    Disagreed,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum SensorChecks{
    Acceptable,
    Unnaceptable,
}

fn main() -> SensorChecks {
    let keyence_data = vec![true, true];
    let sensor_check = check_keyence_agrees(&keyence_data);
    return sensor_check;
}

pub fn check_keyence_agrees(keyence_data: &Vec<bool>) -> SensorChecks {

    if keyence_data[0] != keyence_data[1] {
        println!("Keyence disagreement for two consecutive readings.");

        return SensorChecks::Unnaceptable;
    }

    return SensorChecks::Acceptable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable_success() {
        let keyence_data = vec![true, true];
        let desired_outcome = SensorChecks::Acceptable;
        let result = check_keyence_agrees(&keyence_data);
        assert_eq!(result, desired_outcome);
    }

    #[test]
    fn test_uncceptable_success() {
        let keyence_data = vec![true, false];
        let desired_outcome = SensorChecks::Unnaceptable;
        let result = check_keyence_agrees(&keyence_data);
        assert_eq!(result, desired_outcome);
    }
}