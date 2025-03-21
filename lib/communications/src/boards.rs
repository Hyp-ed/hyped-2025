// TODO: macro man this

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum Board {
    Telemetry,
    Navigation,
    Pneumatics,
    Test,
    TemperatureTester,
    KeyenceTester,
    StateMachineTester,
}

impl From<Board> for u8 {
    fn from(val: Board) -> Self {
        match val {
            Board::Telemetry => 0,
            Board::Navigation => 1,
            Board::Pneumatics => 2,
            Board::Test => 3,
            Board::TemperatureTester => 4,
            Board::KeyenceTester => 5,
            Board::StateMachineTester => 6,
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
            4 => Board::TemperatureTester,
            5 => Board::KeyenceTester,
            6 => Board::StateMachineTester,
            _ => panic!("Invalid Board index: {:?}", index),
        }
    }
}
