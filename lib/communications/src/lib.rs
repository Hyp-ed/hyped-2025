use hyped_measurement_ids::gen_measurement_ids;

gen_measurement_ids!("config/pods.yaml", "poddington");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let measurement_id = MeasurementId::Acceleration;
        assert_eq!(measurement_id.to_string(), "acceleration");
        assert_eq!(
            MeasurementId::from_string("acceleration"),
            MeasurementId::Acceleration
        );
        let measurement_id_u8: u8 = measurement_id.into();
        assert_eq!(MeasurementId::Acceleration, measurement_id_u8.into());
    }
}
