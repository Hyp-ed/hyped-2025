use crate::types::State;
use defmt::{info, warn};
use embassy_net::tcp::TcpSocket;
use hyped_core::mqtt::HypedMqttClient;
use rust_mqtt::utils::rng_generator::CountingRng;

pub struct StateMachine<'a> {
    pub(crate) current_state: State,
    pub(crate) mqtt_client: HypedMqttClient<'a, TcpSocket<'a>, CountingRng>,
}

impl<'a> StateMachine<'a> {
    pub fn new(&self, mqtt_client: HypedMqttClient<'a, TcpSocket<'a>, CountingRng>) -> Self {
        
        StateMachine {
            current_state: State::Idle,
            mqtt_client,
        }
    }

    pub fn handle_transition(&mut self, to_state: &State) {
        let transition = State::transition(to_state, &self.current_state);
        match transition {
            Some(transition) => {
                info!("Transitioning from {:?} to {:?}", self.current_state, transition);
                self.current_state = transition;
            }
            None => {
                warn!("Invalid transition requested from {:?} to {:?}", self.current_state, to_state);
            }
        }
    }

    pub async fn run(&mut self) {
        self.mqtt_client.subscribe("stm").await;

        while self.current_state != State::Shutdown {
            let state = self.current_state.to_string();
            self.publish_state("stm", state.as_bytes()).await;

            let new_state = self.consume_state().await;
            self.handle_transition(&new_state);
        }
    }

    pub async fn publish_state(&mut self, topic: &str, payload: &[u8]) {
        self.mqtt_client.send_message(topic, payload, true).await;
    }

    pub async fn consume_state(&mut self) -> State {
        let new_state = self.mqtt_client.receive_message().await.unwrap();
        State::from_string(new_state.1).unwrap()
    }
}
