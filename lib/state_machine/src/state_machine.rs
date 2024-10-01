use crate::types::{SourceAndTarget, State};
use heapless::FnvIndexMap;

pub struct StateMachine {
    pub(crate) current_state: State,
    pub(crate) transition_map: FnvIndexMap<SourceAndTarget, State, 32>, // TODO: bring constant out
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            current_state: State::Idle,
            transition_map: FnvIndexMap::<SourceAndTarget, State, 32>::new(),
        }
        // TODO: populate transition_map (idk if this can be done inplace which is annoying)
    }

    pub fn handle_transition(&mut self, to_state: &State) {
        let to_from_state = SourceAndTarget {
            source: self.current_state,
            target: *to_state,
        };
        if let Some(&new_state) = self.transition_map.get(&to_from_state) {
            self.current_state = new_state;
        }
    }

    pub fn run(&mut self) {
        loop {
            // TODO: consume, update, publish
        }
    }
}
