enum KeyenceDataStatus{
    Agreed,
    disagreed,
}

enum SensorChecks{
    acceptable,
    unnaceptable,
}

fn main() {
    let keyence_data = vec![true, true, false, true, true];
    let sensor_check = check_keyence_agrees(&keyence_data);
    println!("{:?}", sensor_check);
}

pub fn check_keyence_agrees(keyence_data: &Vec<f64>) -> sensor_checks {
    let mut sensor_check = sensor_checks::acceptable;
    let mut previous_data_status = keyence_data_status::agreed;
    let mut current_data_status = keyence_data_status::agreed;

    for i in 0..keyence_data.len()-1 {
        if keyence_data[i] != keyence_data{i+1} {
            current_data_status = keyence_data_status::disagreed;
        }
    }

    previous_data_status = current_data_status;

    if current_data_status == keyence_data_status::disagreed && previous_data_status == keyence_data_status:: disagreed {
        println!("Keyence disagreement for two consecutive readings.");

        sensor_check = sensor_checks::unnaceptable;

        sensor_check
    }

    sensor_check
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceptable_success() {
        let keyence_data = vec![true, true, false, true, true];
        let desired_outcome = sensor_checks::acceptable;
        let result = check_keyence_agrees(keyence_data);
        assert_eq!(result, desired_outcome);
    }
}