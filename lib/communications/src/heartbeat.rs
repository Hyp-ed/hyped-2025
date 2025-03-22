use super::boards::Board;

#[derive(Debug, defmt::Format, Clone, Copy, PartialEq)]
pub struct Heartbeat {
    pub to: Board,
    pub from: Board,
}

impl Heartbeat {
    pub fn new(to: Board, from: Board) -> Self {
        Self { to, from }
    }
}
