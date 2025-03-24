/// Reason for the emergency stop
#[derive(Debug, Clone, Copy, defmt::Format, PartialEq, Eq)]
pub enum Reason {
    Unknown,
}

impl From<Reason> for u8 {
    fn from(val: Reason) -> Self {
        match val {
            Reason::Unknown => 0,
        }
    }
}

impl From<u8> for Reason {
    fn from(index: u8) -> Self {
        match index {
            0 => Reason::Unknown,
            _ => panic!("Invalid Reason index"),
        }
    }
}
