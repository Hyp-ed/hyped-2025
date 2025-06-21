use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, once_lock::OnceLock, watch::Watch,
};
use hyped_communications::boards::Board;
use hyped_state_machine::states::State;

pub static THIS_BOARD: OnceLock<Board> = OnceLock::new();
pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();
pub static EMERGENCY: Watch<CriticalSectionRawMutex, bool, 1> = Watch::new();
