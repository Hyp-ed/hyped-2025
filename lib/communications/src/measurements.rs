use core::str::FromStr;
use heapless::String;
use hyped_measurement_ids::gen_measurement_ids;

use super::{boards::Board, data::CanData};

gen_measurement_ids!("../../config/pods.yaml", "poddington");

#[derive(Debug, PartialEq, Clone, defmt::Format)]
pub struct MeasurementReading {
    pub reading: CanData,
    pub board: Board,
    pub measurement_id: MeasurementId,
}

impl MeasurementReading {
    pub fn new(reading: CanData, board: Board, measurement_id: MeasurementId) -> Self {
        let new = Mea
        MeasurementReading {
            reading,
            board,
            measurement_id,
        }
    }
}
