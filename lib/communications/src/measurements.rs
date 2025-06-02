use hyped_core::config::MeasurementId;

use super::{boards::Board, data::CanData};

#[derive(Debug, PartialEq, Clone, defmt::Format)]
pub struct MeasurementReading {
    pub reading: CanData,
    pub board: Board,
    pub measurement_id: MeasurementId,
}

impl MeasurementReading {
    pub fn new(reading: CanData, board: Board, measurement_id: MeasurementId) -> Self {
        MeasurementReading {
            reading,
            board,
            measurement_id,
        }
    }
}
