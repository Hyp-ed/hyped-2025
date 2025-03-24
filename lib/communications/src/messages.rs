use hyped_can::HypedCanFrame;
use hyped_state_machine::states::State;

use crate::state_transition::StateTransitionCommand;

use super::{
    can_id::CanId,
    data::{CanData, CanDataType},
    heartbeat::Heartbeat,
    identifier::MessageIdentifier,
    measurements::MeasurementReading,
    state_transition::StateTransitionRequest,
};

#[derive(PartialEq, Debug, Clone, defmt::Format)]
pub enum CanMessage {
    MeasurementReading(MeasurementReading),
    StateTransitionCommand(StateTransitionCommand),
    StateTransitionRequest(StateTransitionRequest),
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
                    measurement_reading.reading.into(),
                    message_identifier,
                );
                HypedCanFrame::new(can_id.into(), measurement_reading.reading.into())
            }
            CanMessage::StateTransitionCommand(state_transition) => {
                let can_id = CanId::new(
                    state_transition.from_board,
                    CanDataType::State,
                    MessageIdentifier::StateTransitionCommand,
                );
                HypedCanFrame::new(
                    can_id.into(),
                    CanData::State(state_transition.to_state.into()).into(),
                )
            }
            CanMessage::StateTransitionRequest(state_transition) => {
                let can_id = CanId::new(
                    state_transition.requesting_board,
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

        match message_identifier {
            MessageIdentifier::Measurement(measurement_id) => {
                let reading: CanData = frame.data.into();
                let measurement_reading = MeasurementReading {
                    reading,
                    board,
                    measurement_id,
                };
                CanMessage::MeasurementReading(measurement_reading)
            }
            MessageIdentifier::StateTransitionCommand => {
                let reading: CanData = frame.data.into();
                match reading {
                    CanData::State(state) => {
                        let to_state: State = state.into();
                        let state_transition = StateTransitionCommand::new(board, to_state);
                        CanMessage::StateTransitionCommand(state_transition)
                    }
                    _ => panic!("Invalid CanData for StateTransition"),
                }
            }
            MessageIdentifier::StateTransitionRequest => {
                let reading: CanData = frame.data.into();
                match reading {
                    CanData::State(state) => {
                        let to_state: State = state.into();
                        let state_transition = StateTransitionRequest::new(board, to_state);
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
    use hyped_state_machine::states::State;

    use crate::{
        boards::Board,
        data::CanData,
        heartbeat::Heartbeat,
        measurements::{MeasurementId, MeasurementReading},
        messages::CanMessage,
        state_transition::{StateTransitionCommand, StateTransitionRequest},
    };

    #[test]
    fn it_works() {
        let measurement_reading = MeasurementReading::new(
            CanData::F32(0.0),
            Board::Telemetry,
            MeasurementId::Acceleration,
        );
        let can_message = CanMessage::MeasurementReading(measurement_reading);

        let can_frame: HypedCanFrame = can_message.clone().into();
        let can_message_from_frame: CanMessage = can_frame.into();

        assert_eq!(can_message, can_message_from_frame)
    }

    #[test]
    fn it_works_state_transition_command() {
        let state_transition = StateTransitionCommand::new(Board::Test, State::EmergencyBrake);
        let state_transition = CanMessage::StateTransitionCommand(state_transition);
        let can_frame: HypedCanFrame = state_transition.clone().into();
        let can_message_from_frame: CanMessage = can_frame.into();
        assert_eq!(state_transition, can_message_from_frame)
    }

    #[test]
    fn it_works_state_transition_request() {
        let state_transition = StateTransitionRequest::new(Board::Test, State::EmergencyBrake);
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
