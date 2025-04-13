#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum Board {
    Telemetry = 0,
    Navigation = 1,
    Pneumatics = 2,
    Test = 3,
    TemperatureTester = 4,
    KeyenceTester = 5,
    StateMachineTester = 6,
    Mqtt = 7,
}

impl From<Board> for u8 {
    fn from(board: Board) -> Self {
        board as u8
    }
}

impl TryFrom<u8> for Board {
    type Error = &'static str;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        match index {
            0 => Ok(Board::Telemetry),
            1 => Ok(Board::Navigation),
            2 => Ok(Board::Pneumatics),
            3 => Ok(Board::Test),
            4 => Ok(Board::TemperatureTester),
            5 => Ok(Board::KeyenceTester),
            6 => Ok(Board::StateMachineTester),
            7 => Ok(Board::Mqtt),
            _ => Err("Invalid Board index"),
        }
    }
}
