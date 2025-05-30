use crate::boards::Board;
use hyped_state_machine::states::State;

/// A request to transition to a new state from a given board.
/// Will be input to the state machine, which will output a command to transition to the new state if the request is valid.
#[derive(Debug, PartialEq, Clone, defmt::Format)]
pub struct StateTransitionRequest {
    pub requesting_board: Board,
    pub to_state: State,
}

impl StateTransitionRequest {
    pub fn new(requesting_board: Board, to_state: State) -> Self {
        StateTransitionRequest {
            requesting_board,
            to_state,
        }
    }
}

/// A command to transition to a new state.
/// All boards must obey this command and transition to the new state.
/// `from_board` needed for CAN ID.
#[derive(Debug, PartialEq, Clone, defmt::Format)]
pub struct StateTransitionCommand {
    pub from_board: Board,
    pub to_state: State,
}

impl StateTransitionCommand {
    pub fn new(from_board: Board, to_state: State) -> Self {
        StateTransitionCommand {
            from_board,
            to_state,
        }
    }
}
