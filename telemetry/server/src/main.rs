use axum::Router;
use mqtt::ingestion::MqttIngestionService;
use openmct::data::realtime::MeasurementReading;
use tokio::sync::broadcast;

mod config;
mod mqtt;
mod openmct;

#[derive(Clone)]
pub struct TelemetryServerState {
    influxdb_client: influxdb2::Client,
    realtime_channel: broadcast::Sender<MeasurementReading>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100);

    let state = TelemetryServerState {
        influxdb_client: config::get_influxdb_client(),
        realtime_channel: tx.clone(),
    };

    let app = Router::new()
        .nest("/openmct", openmct::routes::get_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    // Start the MQTT ingestion service
    tokio::spawn(async {
        let mut mqtt_client = MqttIngestionService::new().await;
        mqtt_client.ingest_measurements(tx).await;
    });

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
