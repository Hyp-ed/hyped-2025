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

pub fn main() {
    let keyence_data = vec![true, true, false, true, true];
    let sensor_check = check_keyence_agrees(&keyence_data);
    println!("{:?}", sensor_check);
}

fn check_keyence_agrees(keyence_data: &Vec<bool>) -> SensorChecks {
    let mut sensor_check = SensorChecks::Acceptable;
    let mut data_status = KeyenceDataStatus::Agreed;

    for i in 0..keyence_data.len()-1 {
        if keyence_data[i] != keyence_data[i+1] {
            data_status = KeyenceDataStatus::Disagreed;
        }
    }

    if data_status == KeyenceDataStatus::Disagreed {
        println!("Keyence disagreement for two consecutive readings.");

        sensor_check = SensorChecks::Unnaceptable;

        return sensor_check
    }

    sensor_check
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable_success() {
        let keyence_data = vec![true, true, false, true, true];
        let desired_outcome = SensorChecks::Acceptable;
        let result = check_keyence_agrees(&keyence_data);
        assert_eq!(result, desired_outcome);
    }
}