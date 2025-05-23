/// Reason for the emergency stop
#[derive(Debug, Clone, Copy, defmt::Format, PartialEq, Eq)]
#[repr(u8)]
pub enum Reason {
    Unknown = 1,
    Test = 2,
    CriticalTemperatureLimit = 3,
    NoInitialHeartbeat = 4,
    MissingHeartbeat = 5,
    TemperatureUpperLimitFailure = 6,
    TemperatureLowerLimitFailure = 7,
}

impl TryFrom<u8> for Reason {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Reason::Unknown),
            2 => Ok(Reason::Test),
            3 => Ok(Reason::CriticalTemperatureLimit),
            4 => Ok(Reason::NoInitialHeartbeat),
            5 => Ok(Reason::MissingHeartbeat),
            6 => Ok(Reason::TemperatureUpperLimitFailure),
            7 => Ok(Reason::TemperatureLowerLimitFailure),
            _ => Err("Invalid reason for emergency stop"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reason_conversion() {
        assert_eq!(
            Reason::Unknown,
            Reason::try_from(Reason::Unknown as u8).unwrap()
        );
        assert_eq!(Reason::Test, Reason::try_from(Reason::Test as u8).unwrap());
        assert_eq!(
            Reason::CriticalTemperatureLimit,
            Reason::try_from(Reason::CriticalTemperatureLimit as u8).unwrap()
        );
        assert_eq!(
            Reason::NoInitialHeartbeat,
            Reason::try_from(Reason::NoInitialHeartbeat as u8).unwrap()
        );
        assert_eq!(
            Reason::MissingHeartbeat,
            Reason::try_from(Reason::MissingHeartbeat as u8).unwrap()
        );
        assert_eq!(
            Reason::TemperatureUpperLimitFailure,
            Reason::try_from(Reason::TemperatureUpperLimitFailure as u8).unwrap()
        );
        assert_eq!(
            Reason::TemperatureLowerLimitFailure,
            Reason::try_from(Reason::TemperatureLowerLimitFailure as u8).unwrap()
        );
        assert_eq!(
            Err("Invalid reason for emergency stop"),
            Reason::try_from(8)
        );
    }
}
