#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

pub enum LogTarget {
    Console,
    Mqtt,
}
