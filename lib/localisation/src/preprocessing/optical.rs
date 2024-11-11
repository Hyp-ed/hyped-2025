use heapless::Vec;
use libm::sqrtf;

pub fn process_data(raw_optical_data: Vec<Vec<f64, 2>, 2>) -> Vec<f32, 2>{

    let mut optical_data: Vec<f32, 2> = Vec::from_slice(&[0.0, 0.0]).unwrap();

    for i in 0..1 {
        let mut magnitude: f32 = 0.0;

        for data in raw_optical_data[i] {
            magnitude += data*data;
        }
        optical_data[i] = sqrtf(magnitude);
    }

    optical_data
}