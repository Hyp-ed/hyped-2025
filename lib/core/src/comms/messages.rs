use hyped_can::HypedCanFrame;

use super::{
    can_id::CanId,
    data::CanData,
    measurements::{MeasurementId, MeasurementReading},
};

#[derive(PartialEq, Debug, Clone, defmt::Format)]
pub enum CanMessage {
    MeasurementReading(MeasurementReading),
}

impl Into<HypedCanFrame> for CanMessage {
    fn into(self) -> HypedCanFrame {
        match self {
            CanMessage::MeasurementReading(measurement_reading) => {
                let message_identifier =
                    MessageIdentifier::Measurement(measurement_reading.measurement_id);
                let can_id = CanId::new(
                    measurement_reading.board,
                    measurement_reading.can_data_type,
                    message_identifier,
                );
                HypedCanFrame::new(can_id.into(), measurement_reading.reading.into())
            }
        }
    }
}

impl From<HypedCanFrame> for CanMessage {
    fn from(frame: HypedCanFrame) -> Self {
        let can_id: CanId = frame.can_id.into();
        let message_identifier = can_id.message_identifier;
        let board = can_id.board;
        let can_data_type = can_id.message_type;

        match message_identifier {
            MessageIdentifier::Measurement(measurement_id) => {
                let reading: CanData = frame.data.into();
                let measurement_reading = MeasurementReading {
                    reading,
                    can_data_type,
                    board,
                    measurement_id,
                };
                CanMessage::MeasurementReading(measurement_reading)
            }
        }
    }
}

#[derive(Debug)]
pub enum MessageIdentifier {
    Measurement(MeasurementId),
}

impl Into<u16> for MessageIdentifier {
    fn into(self) -> u16 {
        match self {
            MessageIdentifier::Measurement(measurement_id) => measurement_id.into(),
        }
    }
}

impl From<u16> for MessageIdentifier {
    fn from(id: u16) -> Self {
        MessageIdentifier::Measurement(id.into())
    }
}

#[cfg(test)]
mod tests {
    use hyped_can::HypedCanFrame;

    use crate::comms::{
        boards::Board,
        data::{CanData, CanDataType},
        measurements::{MeasurementId, MeasurementReading},
        messages::CanMessage,
    };

    #[test]
    fn it_works() {
        let measurement_reading = MeasurementReading::new(
            CanData::F32(0.0),
            CanDataType::F32,
            Board::Telemetry,
            MeasurementId::Temperature,
        );
        let can_message = CanMessage::MeasurementReading(measurement_reading);

        let can_frame: HypedCanFrame = can_message.clone().into();
        let can_message_from_frame: CanMessage = can_frame.into();

        assert_eq!(can_message, can_message_from_frame)
    }
}
