use crate::types::{SourceAndTarget, State};
use heapless::FnvIndexMap;
use hyped_core::mqtt::HypedMqttClient;
use embassy_net::tcp::TcpSocket;
use rust_mqtt::utils::rng_generator::CountingRng;

pub struct StateMachine<'a> {
    pub(crate) current_state: State,
    pub(crate) transition_map: FnvIndexMap<SourceAndTarget, State, 32>, // TODO: bring constant out
    pub(crate) mqtt_client: HypedMqttClient<'a, TcpSocket<'a>, CountingRng>, 
}

impl<'a> StateMachine<'a> {
    pub fn new(
        network_driver: TcpSocket<'a>,
        buffer: &'a mut [u8],
        buffer_len: usize,
        recv_buffer: &'a mut [u8],
        recv_buffer_len: usize,
        client_id: &'a str,
    ) -> Self {
        let mut stm = StateMachine {
            current_state: State::Idle,
            transition_map: FnvIndexMap::<SourceAndTarget, State, 32>::new(),
            mqtt_client: HypedMqttClient::new(
                network_driver, 
                buffer,
                buffer_len,
                recv_buffer,
                recv_buffer_len,
                client_id,
            ),
        };
        
        // this is not a very nice way to do this, but it's a start (move out into a function)
        let _ = stm.transition_map.insert(SourceAndTarget {
            source: State::Idle,
            target: State::Calibrate,
        }, State::Calibrate);

        // subscribe to topic

        stm
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
        loop {
            // placeholder
            let state = "idle";
            self.publish_state("state", state.as_bytes()).await;
        }
    }

    pub async fn publish_state(&mut self, topic: &str, payload: &[u8]) {
        self.mqtt_client.send_message(topic, payload, true).await;
    }

    pub async fn consume_state(&mut self) -> State {
        let _msg = self.mqtt_client.receive_message().await;
        State::Idle
    }
}
