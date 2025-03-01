use hyped_core::{
    logging::{info, warn},
    states::State,
};

pub struct StateMachine {
    pub current_state: State,
}

impl<'a> Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> StateMachine {
    pub fn new() -> Self {
        StateMachine {
            current_state: State::Idle,
        }
    }

    pub fn handle_transition(&mut self, to_state: &State) -> Option<State> {
        let transition = State::transition(&self.current_state, to_state);
        match transition {
            Some(transition) => {
                info!(
                    "Transitioning from {:?} to {:?}",
                    self.current_state, transition
                );
                self.current_state = transition;
                Some(transition)
            }
            None => {
                warn!(
                    "Invalid transition requested from {:?} to {:?}",
                    self.current_state, to_state
                );
                None
            }
        }
    }
}
