use hyped_core::types::AccelerometerData;
use hyped_core::types::RawAccelerometerData;
use hyped_core::types::K_NUM_ACCELEROMETERS;
use hyped_core::types::K_NUM_AXIS;


pub struct AccelerometerPreprocessor {
    num_reliable_accelerometers_: i32,
    reliable_accelerometers_: [i32; K_NUM_ACCELEROMETERS as usize],
}

struct Quartiles {
    q1: f32,
    q2: f32,
    q3: f32,
}

trait AcclerometerPreprocessor{
    fn new() -> AccelerometerPreprocessor;

    fn process_data(&self, data: RawAccelerometerData) -> AccelerometerData;
    fn handle_outliers(&self, data: AccelerometerData);
    fn get_quartiles(&self, data: AccelerometerData) -> Quartiles;
}

impl AcclerometerPreprocessor for AccelerometerPreprocessor {
    fn new() -> AccelerometerPreprocessor {
        AccelerometerPreprocessor {
            num_reliable_accelerometers_: 0,
            reliable_accelerometers_: [0; K_NUM_ACCELEROMETERS as usize],
        }
    }

    fn handle_outliers(&self, data: AccelerometerData) {
       let quartiles: Quartiles;
       if (num_reliable_accelerometers_ == K_NUM_ACCELEROMETERS) {
           quartiles = get_quartiles(data);
       }
       else if (num_reliable_accelerometers_ == K_NUM_ACCELEROMETERS - 1) {
           let filtered_data = Box::new([0.0; K_NUM_ACCELEROMETERS-1]);
           core::type::kNumAccelerometers.iter().for_each(|i| {
               if (reliable_accelerometers_.contains(i)) {
                   filtered_data.push(data[i]);
               }
           });
           quartiles = get_quartiles(data);
       }
       else {
           self.num_reliable_accelerometers_ += 1;
       }
    }

    fn get_quartiles(&self, data: AccelerometerData) -> Quartiles {
        let mut sorted_data = data.clone();
        // sorted_data.sort();
        let q1 = sorted_data[K_NUM_ACCELEROMETERS/4];
        let q2 = sorted_data[K_NUM_ACCELEROMETERS/2];
        let q3 = sorted_data[K_NUM_ACCELEROMETERS*3/4];
        return Quartiles{q1, q2, q3};
    }

    fn process_data(&self, data: RawAccelerometerData) -> AccelerometerData{
        // do some processing
        let mut accelerometer_data: AccelerometerData;
        let magnitude: f32;
        
        data
            .iter()
            .enumerate()
            .for_each(|(i, axis)| {
                magnitude = 0.0;
                axis.iter().enumerate().map(|(j, val)| {
                    magnitude += val * val;
                });
                accelerometer_data[i] = magnitude.sqrt();
            });

        let clean_accelerometer_data: AccelerometerData = self.handle_outliers(accelerometer_data);

        clean_accelerometer_data
    }

}

mod tests {
    use super::*;

    #[test]
    fn test_accelerometer_preprocessor() {
        let preprocessor = AccelerometerPreprocessor {
            num_reliable_accelerometers_: 0,
            reliable_accelerometers_: Vec::new(),
        };

        let data = core::types::AccelerometerData {
            0: [1.0, 2.0, 3.0],
            1: [4.0, 5.0, 6.0],
            2: [7.0, 8.0, 9.0],
        };

        let processed_data = preprocessor.processData(data);

        assert_eq!(processed_data[0], [2.0, 4.0, 6.0]);
        assert_eq!(processed_data[1], [8.0, 10.0, 12.0]);
        assert_eq!(processed_data[2], [14.0, 16.0, 18.0]);
    }
}
