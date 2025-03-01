use crate::states::State;

use super::boards::Board;

#[derive(Debug, PartialEq, Clone, defmt::Format)]
pub struct StateTransition {
    pub board: Board,
    pub to_state: State,
}

impl StateTransition {
    pub fn new(board: Board, to_state: State) -> Self {
        StateTransition { board, to_state }
    }
}
