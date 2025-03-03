use hyped_can::HypedCanFrame;

use crate::states::State;

use super::{
    can_id::CanId,
    data::{CanData, CanDataType},
    heartbeat::Heartbeat,
    identifier::MessageIdentifier,
    measurements::MeasurementReading,
    state_transition::StateTransition,
};

#[derive(PartialEq, Debug, Clone, defmt::Format)]
pub enum CanMessage {
    MeasurementReading(MeasurementReading),
    StateTransition(StateTransition),
    StateTransitionRequest(StateTransition),
    Heartbeat(Heartbeat),
}

// Converts a CanMessage into a HypedCanFrame ready to be sent over the CAN bus
impl From<CanMessage> for HypedCanFrame {
    fn from(val: CanMessage) -> Self {
        match val {
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
            CanMessage::StateTransition(state_transition) => {
                let can_id = CanId::new(
                    state_transition.board,
                    CanDataType::State,
                    MessageIdentifier::StateTransition,
                );
                HypedCanFrame::new(
                    can_id.into(),
                    CanData::State(state_transition.to_state.into()).into(),
                )
            }
            CanMessage::StateTransitionRequest(state_transition) => {
                let can_id = CanId::new(
                    state_transition.board,
                    CanDataType::State,
                    MessageIdentifier::StateTransitionRequest,
                );
                HypedCanFrame::new(
                    can_id.into(),
                    CanData::State(state_transition.to_state.into()).into(),
                )
            }
            CanMessage::Heartbeat(heartbeat) => {
                let can_id = CanId::new(
                    heartbeat.from,
                    CanDataType::Heartbeat,
                    MessageIdentifier::Heartbeat,
                );
                HypedCanFrame::new(can_id.into(), CanData::Heartbeat(heartbeat.to).into())
            }
        }
    }
}

// Converts an incoming HypedCanFrame read from the CAN bus into a CanMessage
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
            MessageIdentifier::StateTransition => {
                let reading: CanData = frame.data.into();
                match reading {
                    CanData::State(state) => {
                        let to_state: State = state.into();
                        let state_transition = StateTransition::new(board, to_state);
                        CanMessage::StateTransition(state_transition)
                    }
                    _ => panic!("Invalid CanData for StateTransition"),
                }
            }
            MessageIdentifier::StateTransitionRequest => {
                let reading: CanData = frame.data.into();
                match reading {
                    CanData::State(state) => {
                        let to_state: State = state.into();
                        let state_transition = StateTransition::new(board, to_state);
                        CanMessage::StateTransitionRequest(state_transition)
                    }
                    _ => panic!("Invalid CanData for StateTransitionRequest"),
                }
            }
            MessageIdentifier::Heartbeat => {
                let reading: CanData = frame.data.into();
                match reading {
                    CanData::Heartbeat(to) => {
                        let heartbeat = Heartbeat::new(to, board);
                        CanMessage::Heartbeat(heartbeat)
                    }
                    _ => panic!("Invalid CanData for Heartbeat"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hyped_can::HypedCanFrame;

    use crate::{
        comms::{
            boards::Board,
            data::{CanData, CanDataType},
            heartbeat::Heartbeat,
            measurements::{MeasurementId, MeasurementReading},
            messages::CanMessage,
            state_transition::StateTransition,
        },
        states::State,
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

    #[test]
    fn it_works_state_transition() {
        let state_transition = StateTransition::new(Board::Test, State::EmergencyBrake);
        let state_transition = CanMessage::StateTransition(state_transition);
        let can_frame: HypedCanFrame = state_transition.clone().into();
        let can_message_from_frame: CanMessage = can_frame.into();
        assert_eq!(state_transition, can_message_from_frame)
    }

    #[test]
    fn it_works_state_transition_request() {
        let state_transition = StateTransition::new(Board::Test, State::EmergencyBrake);
        let state_transition = CanMessage::StateTransitionRequest(state_transition);
        let can_frame: HypedCanFrame = state_transition.clone().into();
        let can_message_from_frame: CanMessage = can_frame.into();
        assert_eq!(state_transition, can_message_from_frame)
    }

    #[test]
    fn it_works_heartbeat() {
        let heartbeat = CanMessage::Heartbeat(Heartbeat::new(Board::KeyenceTester, Board::Test));
        let can_frame: HypedCanFrame = heartbeat.clone().into();
        let can_message_from_frame: CanMessage = can_frame.into();
        assert_eq!(heartbeat, can_message_from_frame)
    }
}
