use crate::types::State;
use defmt::{info, warn};
use embassy_net::tcp::TcpSocket;
use hyped_core::mqtt::HypedMqttClient;
use rust_mqtt::utils::rng_generator::CountingRng;

pub struct StateMachine<'a> {
    pub current_state: State,
    pub mqtt_client: HypedMqttClient<'a, TcpSocket<'a>, CountingRng>,
}

impl<'a> StateMachine<'a> {
    pub fn new(&self, mqtt_client: HypedMqttClient<'a, TcpSocket<'a>, CountingRng>) -> Self {
        StateMachine {
            current_state: State::Idle,
            mqtt_client,
        }
    }

    pub fn handle_transition(&mut self, to_state: &State) {
        let transition = State::transition(&self.current_state, to_state);
        match transition {
            Some(transition) => {
                info!(
                    "Transitioning from {:?} to {:?}",
                    self.current_state, transition
                );
                self.current_state = transition;
            }
            None => {
                warn!(
                    "Invalid transition requested from {:?} to {:?}",
                    self.current_state, to_state
                );
            }
        }
    }
}
