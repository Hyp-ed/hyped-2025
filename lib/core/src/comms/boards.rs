#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum Board {
    Telemetry,
    Navigation,
    Pneumatics,
    Test,
}

impl Into<u8> for Board {
    fn into(self) -> u8 {
        match self {
            Board::Telemetry => 0,
            Board::Navigation => 1,
            Board::Pneumatics => 2,
            Board::Test => 3,
        }
    }
}

impl From<u8> for Board {
    fn from(index: u8) -> Self {
        match index {
            0 => Board::Telemetry,
            1 => Board::Navigation,
            2 => Board::Pneumatics,
            3 => Board::Test,
            _ => panic!("Invalid Board index: {:?}", index),
        }
    }
}
