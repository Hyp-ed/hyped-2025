use super::boards::Board;

#[derive(Debug, defmt::Format, Clone, Copy, PartialEq)]
pub struct Heartbeat {
    pub to: Board,
    pub from: Board,
    pub timestamp: u32,
}

impl Heartbeat {
    pub fn new(to: Board, from: Board) -> Self {
        Self {
            to,
            from,
            timestamp: 0,
        }
    }
}
