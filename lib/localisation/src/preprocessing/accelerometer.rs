//test
pub core::types

trait AcclerometerPreprocessor{
    num_reliable_accelerometers_: i32;
    reliable_accelerometers_: Vec<i32>;

    fn processData(&self, data: core::types::AccelerometerData) -> core::types::AccelerometerData;

    fn(handleOutliers(&self, data: core::types::AccelerometerData) -> core::types::AccelerometerData;
}

impl AcclerometerPreprocessor for core::types::AccelerometerData{
    fn processData(&self, data: core::types::AccelerometerData) -> core::types::AccelerometerData{
        // do some processing
        let accelerometer_data: core::types::AccelerometerData;
        
        core::types::kNumAccelerometers.iter().for_each(|i| {
            data[i] = data[i] * 2;
            magnitude: f32 = 0.0;
            core::type::kNumAxis.iter().for_each(|j| {
                magnitude += data[i][j] * data[i][j];
            });
            accelerometer_data[i] = magnitude.sqrt();
        });

        const clean_accelerometer_data: core::types::AccelerometerData = handleOutliers(accelerometer_data);

        return data;
    }

    fn handleOutliers(&self, data: core::types::AccelerometerData) {
        quartiles: Quartiles;
       if (num_reliable_accelerometers_ == core::types::kNumAccelerometers) {
           quartiles = getQuartiles(data);
       }
       else if (num_reliable_accelerometers_ == core::types::kNumAccelerometers - 1) {
           let filtered_data = Box::new([0.0; core::types::kNumAccelerometers-1]);
           core::type::kNumAccelerometers.iter().for_each(|i| {
               if (reliable_accelerometers_.contains(i)) {
                   filtered_data.push(data[i]);
               }
           });
           quartiles = getQuartiles(data);
       }
       else {
           num_reliable_accelerometers_++;
       }
    }
}
