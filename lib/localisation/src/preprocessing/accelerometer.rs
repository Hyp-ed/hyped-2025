use crate::types::{
    AccelerometerData, RawAccelerometerData, SensorChecks, NUM_ACCELEROMETERS,
    NUM_ALLOWED_ACCELEROMETER_OUTLIERS, NUM_AXIS,
};
use heapless::Vec;
use libm;

/// Stores the quartiles of the data and the bounds for outliers
/// which are calculated from the quartiles
#[allow(dead_code)]
pub struct Quartiles {
    q1: f32,
    q2: f32,
    q3: f32,

    iqr: f32,
    lower_bound: f32,
    upper_bound: f32,
}

/// Implementation of the Quartiles struct which calculates the bounds for outliers
impl Quartiles {
    pub fn new(q1: f32, q2: f32, q3: f32, is_unreliable: bool) -> Self {
        let bound_factor = if is_unreliable { 1.2 } else { 1.5 };

        Self {
            q1,
            q2,
            q3,
            iqr: q3 - q1,
            lower_bound: q1 - bound_factor * (q3 - q1),
            upper_bound: q3 + bound_factor * (q3 - q1),
        }
    }
}

/// Responsible for processing accelerometer data and removing outliers
#[derive(Default)]
pub struct AccelerometerPreprocessor {
    /// number of true values in reliable_accelerometers
    num_reliable_accelerometers: i32,
    /// true if accelerometer at index is reliable
    reliable_accelerometers: [bool; NUM_ACCELEROMETERS],
    /// number of outliers detected for each accelerometer
    num_outliers_per_accelerometer: [i32; NUM_ACCELEROMETERS],
}

impl AccelerometerPreprocessor {
    /// Creates a new AccelerometerPreprocessor
    /// By default, all accelerometers are deemed as reliable
    pub fn new() -> Self {
        Self {
            num_reliable_accelerometers: NUM_ACCELEROMETERS as i32,
            reliable_accelerometers: [true; NUM_ACCELEROMETERS],
            num_outliers_per_accelerometer: [0; NUM_ACCELEROMETERS],
        }
    }

    /// Removes points in data that are deemed as outliers
    /// This is based on bounds calculated from the quartiles of the data
    /// Any points from unreliable accelerometers or those that are out of bounds
    /// are replaced with the median of the data
    pub fn handle_outliers(
        &mut self,
        data: AccelerometerData<NUM_ACCELEROMETERS>,
    ) -> Option<AccelerometerData<NUM_ACCELEROMETERS>> {
        let quartiles = self.calculate_quartiles(&data)?;

        let accelerometer_data = data
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                if !self.reliable_accelerometers[i] {
                    quartiles.q2
                } else if val < quartiles.lower_bound || val > quartiles.upper_bound {
                    self.num_outliers_per_accelerometer[i] += 1;
                    quartiles.q2
                } else {
                    self.num_outliers_per_accelerometer[i] = 0;
                    val
                }
            })
            .collect();

        Some(accelerometer_data)
    }

    /// Calculates the quartiles of the data
    /// If all accelerometers are reliable, the quartiles are calculated normally
    /// If one accelerometer is unreliable, the quartiles are calculated with the unreliable
    /// accelerometer removed
    /// If more than one accelerometer is unreliable, None is returned
    pub fn calculate_quartiles(
        &self,
        data: &AccelerometerData<NUM_ACCELEROMETERS>,
    ) -> Option<Quartiles> {
        if self.num_reliable_accelerometers == NUM_ACCELEROMETERS as i32 {
            Some(self.get_quartiles(data))
        } else if self.num_reliable_accelerometers == (NUM_ACCELEROMETERS as i32 - 1) {
            const SIZE: usize = NUM_ACCELEROMETERS - 1;
            let filtered_data: AccelerometerData<SIZE> = data
                .iter()
                .enumerate()
                .filter(|(i, _)| self.reliable_accelerometers[*i])
                .map(|(_, val)| *val)
                .collect();
            Some(self.get_quartiles(&filtered_data))
        } else {
            None
        }
    }

    /// Main function to process accelerometer data
    /// This function calculates the magnitude of the acceleration (across all axes) for each
    /// accelerometer
    /// It then removes outliers from the data and checks if the data is reliable
    /// Unreliable data is deemed unacceptable and the function returns None
    pub fn process_data(
        &mut self,
        data: RawAccelerometerData<NUM_ACCELEROMETERS, NUM_AXIS>,
    ) -> Option<AccelerometerData<NUM_ACCELEROMETERS>> {
        let accelerometer_data: AccelerometerData<NUM_ACCELEROMETERS> = data
            .iter()
            .map(|axis| libm::sqrtf(axis.iter().fold(0.0, |acc, val| acc + val * val)))
            .collect();
        let clean_accelerometer_data = self.handle_outliers(accelerometer_data)?;

        if self.check_reliable() == SensorChecks::Unacceptable {
            return None;
        }

        Some(clean_accelerometer_data)
    }

    /// Sets accelerometers as unreliable if they have more than
    /// NUM_ALLOWED_ACCELEROMETER_OUTLIERS outliers detected
    /// Deems the data unacceptable if more than 1 accelerometer is unreliable
    pub fn check_reliable(&mut self) -> SensorChecks {
        self.num_outliers_per_accelerometer
            .iter()
            .enumerate()
            .for_each(|(i, val)| {
                if self.reliable_accelerometers[i]
                    && val >= &(NUM_ALLOWED_ACCELEROMETER_OUTLIERS as i32)
                {
                    self.reliable_accelerometers[i] = false;
                    self.num_reliable_accelerometers -= 1;
                }
            });

        if self.num_reliable_accelerometers < NUM_ACCELEROMETERS as i32 - 1 {
            return SensorChecks::Unacceptable;
        }

        SensorChecks::Acceptable
    }

    pub fn get_quartiles<const SIZE: usize>(&self, data: &AccelerometerData<SIZE>) -> Quartiles {
        // Clone and sort data
        let mut sorted_data = data.clone();
        sorted_data
            .as_mut_slice()
            .sort_by(|a, b| a.partial_cmp(b).unwrap());

        let quartile_keys: [f32; 3] = [0.25, 0.5, 0.75];
        let mut quartiles: [f32; 3] = [0.0; 3];

        for (i, &quartile) in quartile_keys.iter().enumerate() {
            let index_quartile = (1.0 + self.num_reliable_accelerometers as f32) * quartile;

            let index_quartile_floor = libm::floorf(index_quartile) as usize - 1;
            let index_quartile_ceil = libm::ceilf(index_quartile) as usize - 1;

            quartiles[i] = (data.get(index_quartile_floor).unwrap_or(&0.0)
                + data.get(index_quartile_ceil).unwrap_or(&0.0))
                / 2.0;
        }

        Quartiles::new(
            quartiles[0],
            quartiles[1],
            quartiles[2],
            self.num_reliable_accelerometers < NUM_ACCELEROMETERS as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libm;

    #[test]
    pub fn test_process_data() {
        let mut preprocessor = AccelerometerPreprocessor::new();

        assert_eq!(preprocessor.num_reliable_accelerometers, 4);
        assert_eq!(
            preprocessor.reliable_accelerometers,
            [true; NUM_ACCELEROMETERS]
        );
        assert_eq!(
            preprocessor.num_outliers_per_accelerometer,
            [0; NUM_ACCELEROMETERS]
        );

        let raw_data: RawAccelerometerData<NUM_ACCELEROMETERS, NUM_AXIS> =
            RawAccelerometerData::from_slice(&[
                Vec::from_slice(&[1.0, 2.0, 3.0]).unwrap(), // sqrt(14) ≈ 3.74
                Vec::from_slice(&[4.0, 5.0, 6.0]).unwrap(), // sqrt(77) ≈ 8.77
                Vec::from_slice(&[7.0, 8.0, 9.0]).unwrap(), // sqrt(194) ≈ 13.93
                Vec::from_slice(&[10.0, 11.0, 12.0]).unwrap(), // sqrt(365) ≈ 19.1
            ])
            .unwrap();

        let processed_data = preprocessor.process_data(raw_data);
        assert!(processed_data.is_some());

        let processed_data = processed_data.unwrap();
        assert_eq!(processed_data[0], libm::sqrtf(14.0_f32));
        assert_eq!(processed_data[1], libm::sqrtf(77.0_f32));
        assert_eq!(processed_data[2], libm::sqrtf(194.0_f32));
        assert_eq!(processed_data[3], libm::sqrtf(365.0_f32));
    }

    #[test]
    pub fn test_process_data_one_unreliable() {
        let mut preprocessor = AccelerometerPreprocessor::new();

        assert_eq!(preprocessor.num_reliable_accelerometers, 4);
        assert_eq!(
            preprocessor.reliable_accelerometers,
            [true; NUM_ACCELEROMETERS]
        );
        assert_eq!(
            preprocessor.num_outliers_per_accelerometer,
            [0; NUM_ACCELEROMETERS]
        );

        preprocessor.reliable_accelerometers = [true, false, true, true];
        preprocessor.num_reliable_accelerometers = 3;

        let raw_data: RawAccelerometerData<NUM_ACCELEROMETERS, NUM_AXIS> =
            RawAccelerometerData::from_slice(&[
                Vec::from_slice(&[1.0, 2.0, 3.0]).unwrap(), // sqrt(14) ≈ 3.74
                Vec::from_slice(&[4.0, 5.0, 6.0]).unwrap(), // replaced with median (from sqrt(14), sqrt(194), sqrt(365))
                Vec::from_slice(&[7.0, 8.0, 9.0]).unwrap(), // sqrt(194) ≈ 13.93
                Vec::from_slice(&[10.0, 11.0, 12.0]).unwrap(), // sqrt(365) ≈ 19.1
            ])
            .unwrap();

        let processed_data = preprocessor.process_data(raw_data);
        assert!(processed_data.is_some());

        let processed_data = processed_data.unwrap();
        assert_eq!(processed_data[0], libm::sqrtf(14.0_f32));
        assert_eq!(processed_data[1], libm::sqrtf(194.0_f32));
        assert_eq!(processed_data[2], libm::sqrtf(194.0_f32));
        assert_eq!(processed_data[3], libm::sqrtf(365.0_f32));
    }

    #[test]
    pub fn test_get_quartiles() {
        let preprocessor = AccelerometerPreprocessor::new();

        let data: AccelerometerData<NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        let processed_data = preprocessor.get_quartiles(&data);

        assert_eq!(processed_data.q1, 1.5);
        assert_eq!(processed_data.q2, 2.5);
        assert_eq!(processed_data.q3, 3.5);
    }

    #[test]
    fn test_calculate_quartiles_max_reliable() {
        let preprocessor = AccelerometerPreprocessor::new();

        let data: AccelerometerData<NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        let processed_data = preprocessor.calculate_quartiles(&data);

        assert!(processed_data.is_some());

        let processed_data = processed_data.unwrap();
        assert_eq!(processed_data.q1, 1.5);
        assert_eq!(processed_data.q2, 2.5);
        assert_eq!(processed_data.q3, 3.5);
        assert_eq!(processed_data.iqr, 2.0);
        assert_eq!((processed_data.lower_bound * 100.0).round(), -150.0);
        assert_eq!((processed_data.upper_bound * 100.0).round(), 650.0);
    }

    #[test]
    fn test_calculate_quartiles_one_unreliable() {
        let mut preprocessor = AccelerometerPreprocessor::new();
        preprocessor.reliable_accelerometers = [true, false, true, true];
        preprocessor.num_reliable_accelerometers = 3;

        let data: AccelerometerData<NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        let processed_data = preprocessor.calculate_quartiles(&data);

        assert!(processed_data.is_some());

        let processed_data = processed_data.unwrap();
        assert_eq!(processed_data.q1, 1.0);
        assert_eq!(processed_data.q2, 3.0);
        assert_eq!(processed_data.q3, 4.0);
        assert_eq!(processed_data.iqr, 3.0);
        assert_eq!((processed_data.lower_bound * 100.0).round(), -260.0);
        assert_eq!((processed_data.upper_bound * 100.0).round(), 760.0);
    }

    #[test]
    fn test_handle_outlier_replace_median() {
        let mut preprocessor = AccelerometerPreprocessor::new();
        preprocessor.reliable_accelerometers = [true, false, true, true];
        preprocessor.num_reliable_accelerometers = 3;

        let data: AccelerometerData<NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 10.0]).unwrap();
        let processed_data = preprocessor.handle_outliers(data);

        assert!(processed_data.is_some());

        let processed_data: AccelerometerData<NUM_ACCELEROMETERS> = processed_data.unwrap();
        assert_eq!(processed_data[0], 1.0);
        assert_eq!(processed_data[1], 3.0); // replace unreliable with median
        assert_eq!(processed_data[2], 3.0);
        assert_eq!(processed_data[3], 10.0);
    }

    #[test]
    fn test_handle_outliers_no_quartiles() {
        let mut preprocessor = AccelerometerPreprocessor::new();
        preprocessor.reliable_accelerometers = [true, false, false, true];
        preprocessor.num_reliable_accelerometers = 2;

        let data: AccelerometerData<NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 10.0]).unwrap();
        let processed_data = preprocessor.handle_outliers(data);

        assert!(processed_data.is_none());
    }
}
