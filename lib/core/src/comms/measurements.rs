use super::{
    boards::Board,
    data::{CanData, CanDataType},
};

#[derive(Debug, PartialEq, Clone, defmt::Format)]
pub struct MeasurementReading {
    pub reading: CanData,
    pub can_data_type: CanDataType,
    pub board: Board,
    pub measurement_id: MeasurementId,
}

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum MeasurementId {
    Temperature,
    Test,
    KeyenceStripeCount,
}

impl From<MeasurementId> for u16 {
    fn from(val: MeasurementId) -> Self {
        match val {
            MeasurementId::Temperature => 0x00,
            MeasurementId::Test => 0x01,
            MeasurementId::KeyenceStripeCount => 0x02,
        }
    }
}

impl From<u16> for MeasurementId {
    fn from(id: u16) -> Self {
        match id {
            0x00 => MeasurementId::Temperature,
            0x01 => MeasurementId::Test,
            0x02 => MeasurementId::KeyenceStripeCount,
            _ => panic!("Invalid MeasurementId"),
        }
    }
}

impl MeasurementReading {
    pub fn new(
        reading: CanData,
        can_data_type: CanDataType,
        board: Board,
        measurement_id: MeasurementId,
    ) -> Self {
        MeasurementReading {
            reading,
            can_data_type,
            board,
            measurement_id,
        }
    }
}
