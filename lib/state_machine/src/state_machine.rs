use crate::types::{SourceAndTarget, State};
use embassy_net::tcp::TcpSocket;
use heapless::FnvIndexMap;
use hyped_core::mqtt::HypedMqttClient;
use rust_mqtt::utils::rng_generator::CountingRng;

pub struct StateMachine<'a> {
    pub(crate) current_state: State,
    pub(crate) transition_map: FnvIndexMap<SourceAndTarget, State, 32>,
    pub(crate) mqtt_client: HypedMqttClient<'a, TcpSocket<'a>, CountingRng>,
}

impl<'a> StateMachine<'a> {
    pub fn new(mqtt_client: HypedMqttClient<'a, TcpSocket<'a>, CountingRng>) -> Self {
        let mut transition_map = FnvIndexMap::<SourceAndTarget, State, 32>::new();

        // this is not a very nice way to do this, but it's a start (move out into a function consruct_transition_map)
        let _ = transition_map.insert(
            SourceAndTarget {
                source: State::Idle,
                target: State::Calibrate,
            },
            State::Calibrate,
        );

        StateMachine {
            current_state: State::Idle,
            transition_map,
            mqtt_client,
        }
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
