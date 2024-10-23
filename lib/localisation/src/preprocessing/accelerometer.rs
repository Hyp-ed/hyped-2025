use hyped_core::types::{AccelerometerData, RawAccelerometerData, SensorChecks};
use hyped_core::types::{K_NUM_ACCELEROMETERS, K_NUM_ALLOWED_ACCELEROMETER_OUTLIERS, K_NUM_AXIS};

pub struct AccelerometerPreprocessor {
    num_reliable_accelerometers_: i32,
    reliable_accelerometers_: [bool; K_NUM_ACCELEROMETERS as usize],
    num_outliers_per_accelerometer_: [i32; K_NUM_ACCELEROMETERS as usize],
}

struct Quartiles {
    q1: f32,
    q2: f32,
    q3: f32,
}

fn get_quartiles<const SIZE: usize>(data: &AccelerometerData<SIZE>) -> Quartiles {
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q1 = sorted_data[K_NUM_ACCELEROMETERS / 4];
    let q2 = sorted_data[K_NUM_ACCELEROMETERS / 2];
    let q3 = sorted_data[K_NUM_ACCELEROMETERS * 3 / 4];
    return Quartiles { q1, q2, q3 };
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

        let iqr = quartiles.q3 - quartiles.q1;
        let mut lower_bound = quartiles.q1 - 1.5 * iqr;
        let mut upper_bound = quartiles.q3 + 1.5 * iqr;
        if self.num_reliable_accelerometers_ < K_NUM_ACCELEROMETERS as i32 {
            lower_bound = quartiles.q1 - 1.2 * iqr;
            upper_bound = quartiles.q3 + 1.2 * iqr;
        }

        let accelerometer_data = data
            .iter()
            .enumerate()
            .map(|(i, val)| {
                if !self.reliable_accelerometers_[i] {
                    return quartiles.q2;
                } else if val < &lower_bound || val > &upper_bound {
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
            quartiles = get_quartiles(data);
        } else if self.num_reliable_accelerometers_ == (K_NUM_ACCELEROMETERS as i32 - 1) {
            const SIZE: usize = K_NUM_ACCELEROMETERS - 1;
            let filtered_data: AccelerometerData<SIZE> = (0..SIZE)
                .into_iter()
                .map(|i| match self.reliable_accelerometers_[i] {
                    true => data[i],
                    false => 0.0,
                })
                .collect();

            quartiles = get_quartiles(&filtered_data);
        } else {
            return None;
        }

        Some(quartiles)
    }

    fn process_data(
        &mut self,
        data: RawAccelerometerData<K_NUM_AXIS, K_NUM_ACCELEROMETERS>,
    ) -> Option<AccelerometerData<K_NUM_ACCELEROMETERS>> {
        // do some processing
        let mut accelerometer_data: AccelerometerData<K_NUM_ACCELEROMETERS> =
            AccelerometerData::new();
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
}

mod tests {
    use super::*;
    use heapless::Vec;

    #[test]
    fn test_accelerometer_preprocessor() {
        let mut preprocessor = AccelerometerPreprocessor {
            num_reliable_accelerometers_: 0,
            reliable_accelerometers_: [true; K_NUM_ACCELEROMETERS as usize],
            num_outliers_per_accelerometer_: [0; K_NUM_ACCELEROMETERS as usize],
        };

        assert_eq!(preprocessor.num_reliable_accelerometers_, 0);
        assert_eq!(
            preprocessor.reliable_accelerometers_,
            [true; K_NUM_ACCELEROMETERS as usize]
        );
        assert_eq!(
            preprocessor.num_outliers_per_accelerometer_,
            [0; K_NUM_ACCELEROMETERS as usize]
        );

        let raw_data: RawAccelerometerData<K_NUM_AXIS, K_NUM_ACCELEROMETERS> =
            RawAccelerometerData::from_slice(&[
                Vec::from_slice(&[1.0, 2.0, 3.0]).unwrap(),
                Vec::from_slice(&[4.0, 5.0, 6.0]).unwrap(),
                Vec::from_slice(&[7.0, 8.0, 9.0]).unwrap(),
            ])
            .unwrap();

        let processed_data = preprocessor.process_data(raw_data);
        assert_eq!(processed_data.is_some(), true);
    }
}
