use crate::types::State;

pub struct StateMachine {
    current_state: State,
    // transition_map: HashMap<SourceAndTarget, State> (use heapless::FnvIndexMap)?
}

impl StateMachine {
    pub fn handle_transition(&mut self) {
        self.current_state = State::KStopped;
    }
}
