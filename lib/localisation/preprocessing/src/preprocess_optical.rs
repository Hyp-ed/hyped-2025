
//TODO: Implement properly!
fn main() {
    let raw_optical_data = vec![vec1[1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    let optical_data = process_optical(&raw_optical_data);
    println!("{:?}", optical_data);
}

pub fn process_optical(raw_optical_data: &Vec<f64>) -> Vec<f64> {

    let mut optical_data = Vec::with_capacity(raw_optical_data.len());

    for i in 0..raw_optical_data.len() {

        let mut magnitude = 0.0;

        for j in 0..raw_optical_data[i].len() {
            magnitude += raw_optical_data[i][j].powi(2);
        }

    }

    optical_data

}