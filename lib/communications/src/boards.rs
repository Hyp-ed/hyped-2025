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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_conversion() {
        assert_eq!(
            Board::Telemetry,
            Board::try_from(Board::Telemetry as u8).unwrap()
        );
        assert_eq!(
            Board::Navigation,
            Board::try_from(Board::Navigation as u8).unwrap()
        );
        assert_eq!(
            Board::Pneumatics,
            Board::try_from(Board::Pneumatics as u8).unwrap()
        );
        assert_eq!(Board::Test, Board::try_from(Board::Test as u8).unwrap());
        assert_eq!(
            Board::TemperatureTester,
            Board::try_from(Board::TemperatureTester as u8).unwrap()
        );
        assert_eq!(
            Board::KeyenceTester,
            Board::try_from(Board::KeyenceTester as u8).unwrap()
        );
        assert_eq!(
            Board::StateMachineTester,
            Board::try_from(Board::StateMachineTester as u8).unwrap()
        );
        assert_eq!(Board::Mqtt, Board::try_from(Board::Mqtt as u8).unwrap());
        assert_eq!(Board::try_from(8), Err("Invalid Board index"));
    }
}
