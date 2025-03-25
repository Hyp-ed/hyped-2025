use core::str::FromStr;
use embassy_futures::join::join;
use heapless::String;
use hyped_communications::{
    boards::Board, messages::CanMessage, state_transition::StateTransitionRequest,
};
use hyped_core::{format, format_string::show, mqtt::MqttMessage, mqtt_topics::MqttTopic};
use hyped_state_machine::states::State;

use super::{
    can::{
        receive::{INCOMING_MEASUREMENTS, INCOMING_STATE_TRANSITION_COMMANDS},
        send::CAN_SEND,
    },
    mqtt::{receive::MQTT_RECEIVE, send::MQTT_SEND},
};

/// Run functions to send CAN messages to MQTT and vice versa.
#[embassy_executor::task]
pub async fn can_to_mqtt() {
    join(
        join(
            send_can_state_transition_command_to_mqtt(),
            send_can_measurement_to_mqtt(),
        ),
        send_mqtt_state_transition_requests_to_can(),
    )
    .await;
}

/// Send a CAN state transition command to MQTT.
pub async fn send_can_state_transition_command_to_mqtt() {
    let state_transition_commands_receiver = INCOMING_STATE_TRANSITION_COMMANDS.receiver();

    loop {
        let state_transition_command = state_transition_commands_receiver.receive().await;

        let message = MqttMessage::new(
            MqttTopic::State,
            String::from_str(state_transition_command.to_state.to_string().as_str()).unwrap(),
        );
        MQTT_SEND.send(message).await;
    }
}

/// Send a CAN measurement to MQTT.
pub async fn send_can_measurement_to_mqtt() {
    let measurements_receiver = INCOMING_MEASUREMENTS.receiver();

    loop {
        let measurement = measurements_receiver.receive().await;

        let topic = measurement
            .measurement_id
            .to_string()
            .parse()
            .expect("Failed to parse measurement ID from CAN bus");

        let message = MqttMessage::new(
            topic,
            String::from_str(format!(&mut [0u8; 1024], "{}", measurement.reading).unwrap())
                .unwrap(),
        );
        MQTT_SEND.send(message).await;
    }
}

/// Send MQTT state transition requests to CAN.
pub async fn send_mqtt_state_transition_requests_to_can() {
    let mqtt_receive_receiver = MQTT_RECEIVE.receiver();

    loop {
        let mqtt_message = mqtt_receive_receiver.receive().await;
        match mqtt_message.topic {
            MqttTopic::StateRequest => {
                let state: State = mqtt_message.payload.as_str().into();

                let can_message = CanMessage::StateTransitionRequest(StateTransitionRequest::new(
                    Board::Mqtt,
                    state,
                ));
                CAN_SEND.send(can_message).await;
            }
            _ => {}
        }
    }
}
