use std::str::from_utf8;

use rumqttc::{
    AsyncClient,
    Event::{Incoming, Outgoing},
    EventLoop, MqttOptions, QoS, SubscribeFilter,
};
use serde::Deserialize;
use tokio::sync::broadcast;

use crate::openmct::data::realtime::MeasurementReading;

pub struct MqttIngestionService {
    client: AsyncClient,
    eventloop: EventLoop,
}

#[derive(Deserialize)]
struct MqttMessage {
    value: f64,
    timestamp: u64,
}

impl MqttIngestionService {
    pub async fn new() -> Self {
        let client_id = "telemetry-server";
        let mqtt_options: MqttOptions = MqttOptions::new(client_id, "localhost", 1883);
        let (client, eventloop) = AsyncClient::new(mqtt_options, 10);
        MqttIngestionService { client, eventloop }
    }

    pub async fn ingest_measurements(
        &mut self,
        realtime_channel: broadcast::Sender<MeasurementReading>,
    ) {
        self.client
            .subscribe_many(vec![SubscribeFilter::new(
                "hyped/+/measurement/+".to_string(),
                QoS::AtLeastOnce,
            )])
            .await
            .unwrap();

        println!("Subscribed to topic");

        while let Ok(notification) = self.eventloop.poll().await {
            match notification {
                Incoming(stuff) => match stuff {
                    rumqttc::Packet::Publish(publish) => {
                        println!("Received publish: {:?}", publish);
                        let decoded = from_utf8(&publish.payload).unwrap();
                        let json: MqttMessage = serde_json::from_str(decoded).unwrap();
                        let pod_id = publish.topic.split('/').nth(1).unwrap();
                        let measurement_key = publish.topic.split('/').nth(3).unwrap();
                        let _ = realtime_channel.send(MeasurementReading::new(
                            pod_id,
                            measurement_key,
                            json.value,
                            json.timestamp,
                        ));
                    }
                    _ => (),
                },
                Outgoing(_) => (),
            }
        }
    }
}
