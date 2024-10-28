use heapless::Vec;
use hyped_core::types::{AccelerometerData, RawAccelerometerData, SensorChecks};
use hyped_core::types::{K_NUM_ACCELEROMETERS, K_NUM_ALLOWED_ACCELEROMETER_OUTLIERS, K_NUM_AXIS};

use std::io::{stdout, Write};

#[derive(Debug)]
pub struct AccelerometerPreprocessor {
    num_reliable_accelerometers_: i32,
    reliable_accelerometers_: [bool; K_NUM_ACCELEROMETERS as usize],
    num_outliers_per_accelerometer_: [i32; K_NUM_ACCELEROMETERS as usize],
}

struct Quartiles {
    q1: f32,
    q2: f32,
    q3: f32,

    iqr: f32,
    lower_bound: f32,
    upper_bound: f32,
}

impl Quartiles {
    fn new(q1: f32, q2: f32, q3: f32, is_unreliable: bool) -> Self {
        let mut bound_factor = 1.5;
        if is_unreliable {
            bound_factor = 1.2;
        }

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

impl AccelerometerPreprocessor {
    fn handle_outliers<const SIZE: usize>(
        &mut self,
        data: AccelerometerData<K_NUM_ACCELEROMETERS>,
    ) -> Option<AccelerometerData<SIZE>> {
        let quartiles = match self.calculate_quartiles(&data) {
            Some(quartiles) => quartiles,
            None => return None,
        };

        let accelerometer_data = data
            .iter()
            .enumerate()
            .map(|(i, val)| {
                if !self.reliable_accelerometers_[i] {
                    return quartiles.q2;
                } else if val < &quartiles.lower_bound || val > &quartiles.upper_bound {
                    self.num_outliers_per_accelerometer_[i] += 1;
                    return quartiles.q2;
                } else {
                    self.num_outliers_per_accelerometer_[i] = 0;
                    return val.clone();
                }
            })
            .collect();

        Some(accelerometer_data)
    }

    fn calculate_quartiles(
        &self,
        data: &AccelerometerData<K_NUM_ACCELEROMETERS>,
    ) -> Option<Quartiles> {
        let quartiles: Quartiles;
        if self.num_reliable_accelerometers_ == K_NUM_ACCELEROMETERS as i32 {
            quartiles = self.get_quartiles(data);
        } else if self.num_reliable_accelerometers_ == (K_NUM_ACCELEROMETERS as i32 - 1) {
            const SIZE: usize = K_NUM_ACCELEROMETERS - 1;
            let mut filtered_data: AccelerometerData<SIZE> =
                AccelerometerData::from_slice(&[0.0; SIZE]).unwrap();
            let mut filtered_data_idx = 0;
            data.iter().enumerate().for_each(|(i, val)| {
                if self.reliable_accelerometers_[i] {
                    filtered_data[filtered_data_idx] = val.clone();
                    filtered_data_idx += 1;
                }
            });
            quartiles = self.get_quartiles(&filtered_data);
        } else {
            return None;
        }

        Some(quartiles)
    }

    fn process_data(
        &mut self,
        data: RawAccelerometerData<K_NUM_ACCELEROMETERS, K_NUM_AXIS>,
    ) -> Option<AccelerometerData<K_NUM_ACCELEROMETERS>> {
        let mut accelerometer_data: AccelerometerData<K_NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[0.0; K_NUM_ACCELEROMETERS as usize]).unwrap();
        let mut magnitude: f32 = 0.0;

        data.iter().enumerate().for_each(|(i, axis)| {
            magnitude = 0.0;
            axis.iter().for_each(|val| {
                magnitude += val * val;
            });
            accelerometer_data[i] = magnitude.sqrt();
        });

        let clean_accelerometer_data = match self.handle_outliers(accelerometer_data) {
            Some(data) => data,
            None => return None,
        };

        if self.check_reliable() == SensorChecks::Unacceptable {
            return None;
        }

        Some(clean_accelerometer_data)
    }

    fn check_reliable(&mut self) -> SensorChecks {
        self.num_outliers_per_accelerometer_
            .iter()
            .enumerate()
            .for_each(|(i, val)| {
                if self.reliable_accelerometers_[i] && val >= &K_NUM_ALLOWED_ACCELEROMETER_OUTLIERS
                {
                    self.reliable_accelerometers_[i] = false;
                    self.num_reliable_accelerometers_ -= 1;
                }
            });

        if self.num_reliable_accelerometers_ < K_NUM_ACCELEROMETERS as i32 - 1 {
            return SensorChecks::Unacceptable;
        }

        SensorChecks::Acceptable
    }

    fn get_quartiles<const SIZE: usize>(&self, data: &AccelerometerData<SIZE>) -> Quartiles {
        let mut sorted_data = data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let quartile_keys: Vec<f32, 3> = Vec::from_slice(&[0.25, 0.5, 0.75]).unwrap();
        let quartiles: Vec<f32, 3> = quartile_keys
            .iter()
            .map(|quartile| {
                let index_quartile: f32 =
                    (1.0 + self.num_reliable_accelerometers_ as f32) * quartile;
                let index_quartile_floor = index_quartile.floor() as usize - 1;
                let index_quartile_ceil = index_quartile.ceil() as usize - 1;

                (data.get(index_quartile_floor).unwrap_or_else(|| &0.0)
                    + data.get(index_quartile_ceil).unwrap_or_else(|| &0.0))
                    / 2.0
            })
            .collect();

        Quartiles::new(
            quartiles[0],
            quartiles[1],
            quartiles[2],
            self.num_reliable_accelerometers_ < K_NUM_ACCELEROMETERS as i32,
        )
    }
}

mod tests {
    use super::*;

    fn default_preprocessor() -> AccelerometerPreprocessor {
        AccelerometerPreprocessor {
            num_reliable_accelerometers_: K_NUM_ACCELEROMETERS as i32,
            reliable_accelerometers_: [true; K_NUM_ACCELEROMETERS as usize],
            num_outliers_per_accelerometer_: [0; K_NUM_ACCELEROMETERS as usize],
        }
    }

    #[test]
    fn test_returns_okay() {
        let mut preprocessor = default_preprocessor();

        assert_eq!(preprocessor.num_reliable_accelerometers_, 4);
        assert_eq!(
            preprocessor.reliable_accelerometers_,
            [true; K_NUM_ACCELEROMETERS as usize]
        );
        assert_eq!(
            preprocessor.num_outliers_per_accelerometer_,
            [0; K_NUM_ACCELEROMETERS as usize]
        );

        let raw_data: RawAccelerometerData<K_NUM_ACCELEROMETERS, K_NUM_AXIS> =
            RawAccelerometerData::from_slice(&[
                Vec::from_slice(&[1.0, 2.0, 3.0]).unwrap(), // sqrt(14) = 3.74
                Vec::from_slice(&[4.0, 5.0, 6.0]).unwrap(), // sqrt(77) = 8.77
                Vec::from_slice(&[7.0, 8.0, 9.0]).unwrap(), // sqrt(194) = 13.93
                Vec::from_slice(&[10.0, 11.0, 12.0]).unwrap(), // sqrt(365) = 19.1
            ])
            .unwrap();

        let processed_data = preprocessor.process_data(raw_data);
        assert_eq!(processed_data.is_some(), true);
    }

    #[test]
    fn test_get_quartiles() {
        let mut preprocessor = default_preprocessor();

        let data: AccelerometerData<K_NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        let processed_data = preprocessor.get_quartiles(&data);

        assert_eq!(processed_data.q1, 1.5);
        assert_eq!(processed_data.q2, 2.5);
        assert_eq!(processed_data.q3, 3.5);
    }

    #[test]
    fn test_calculate_quartiles_max_reliable() {
        let mut preprocessor = default_preprocessor();

        let data: AccelerometerData<K_NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        let processed_data = preprocessor.calculate_quartiles(&data);

        assert_eq!(processed_data.is_some(), true);

        let processed_data = processed_data.unwrap();
        assert_eq!(processed_data.q1, 1.5);
        assert_eq!(processed_data.q2, 2.5);
        assert_eq!(processed_data.q3, 3.5);
        assert_eq!(processed_data.iqr, 2.0);
        assert_eq!(processed_data.lower_bound, -1.5);
        assert_eq!(processed_data.upper_bound, 6.5);
    }

    #[test]
    fn test_calculate_quartiles_one_unreliable() {
        let mut preprocessor = default_preprocessor();
        preprocessor.reliable_accelerometers_ = [true, false, true, true];
        preprocessor.num_reliable_accelerometers_ = 3;

        let data: AccelerometerData<K_NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 4.0]).unwrap();
        let processed_data = preprocessor.calculate_quartiles(&data);

        assert_eq!(processed_data.is_some(), true);

        let processed_data = processed_data.unwrap();
        assert_eq!(processed_data.q1, 1.0);
        assert_eq!(processed_data.q2, 3.0);
        assert_eq!(processed_data.q3, 4.0);
    }

    #[test]
    fn test_handle_outliers() {
        let mut preprocessor = default_preprocessor();

        let data: AccelerometerData<K_NUM_ACCELEROMETERS> =
            AccelerometerData::from_slice(&[1.0, 2.0, 3.0, 10.0]).unwrap();
        let processed_data = preprocessor.handle_outliers(data);

        assert_eq!(processed_data.is_some(), true);

        let processed_data: AccelerometerData<K_NUM_ACCELEROMETERS> = processed_data.unwrap();
        assert_eq!(processed_data[0], 1.0);
        assert_eq!(processed_data[1], 2.0);
        assert_eq!(processed_data[2], 3.0);
        assert_eq!(processed_data[3], 10.0);
    }
}
