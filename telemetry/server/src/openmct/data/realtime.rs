use std::{sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::TelemetryServerState;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeasurementReading {
    pod_id: String,
    measurement_key: String,
    value: f64,
    timestamp: u64, // Store timestamp in nanoseconds
}

impl MeasurementReading {
    pub fn new(pod_id: &str, measurement_key: &str, value: f64, timestamp: u64) -> Self {
        Self {
            pod_id: pod_id.to_string(),
            measurement_key: measurement_key.to_string(),
            value,
            timestamp,
        }
    }
}

pub fn get_routes() -> Router<TelemetryServerState> {
    Router::new().route("/", get(handle_upgrade))
}

async fn handle_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<TelemetryServerState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(ws: WebSocket, state: TelemetryServerState) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = ws.split();

    println!("Client connected");

    // Create a list of rooms that the client is subscribed to which can be used by both tasks at the same time.
    let rooms: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let send_rooms = rooms.clone();

    // Spawn a task that receives subscription messages from the client and
    // updates the client's subscription list.
    let mut subscribe_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            println!("Received message: {}", text);
            let message: Value = serde_json::from_str(&text).unwrap();
            let room = message["room"].as_str().unwrap().to_string();
            let mut unlocked_rooms = rooms.lock().await;
            if message["subscribe"].as_bool().unwrap() {
                unlocked_rooms.push(room);
            } else {
                unlocked_rooms.retain(|r| r != &room);
            }
        }
    });

    let mut rx = state.realtime_channel.subscribe();

    // Spawn a task that sends messages to the client when new data is available,
    // but only if the client is subscribed to the data.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // let reading: MeasurementReading = serde_json::from_str(&msg).unwrap();
            // let unlocked_rooms = send_rooms.lock().await;
            // if unlocked_rooms.contains(&reading.measurement_key) {
            //     println!("Sending message: {}", msg);
            //     let msg = serde_json::to_string(&reading).unwrap();
            //     sender.send(Message::Text(msg)).await.unwrap();
            // }
            println!("Sending message: {:?}", msg);
            sender
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await
                .unwrap();
        }
    });

    // task that adds messages to the broadcast channel
    // tokio::spawn(async move {
    //     let mut interval = tokio::time::interval(Duration::from_secs(1));
    //     loop {
    //         interval.tick().await;
    //         let reading = MeasurementReading {
    //             pod_id: "pod-1".to_string(),
    //             measurement_key: "temperature".to_string(),
    //             value: 25.0,
    //             timestamp: 0,
    //         };
    //         let msg = serde_json::to_string(&reading).unwrap();
    //         println!("Add message to channel");
    //         println!(
    //             "[realtime] Receiver count: {:?}",
    //             state.realtime_channel.receiver_count()
    //         );
    //         state.realtime_channel.send(msg).unwrap();
    //     }
    // });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = &mut subscribe_task => send_task.abort(),
        _ = &mut send_task => subscribe_task.abort(),
    };
}
