use axum::Router;
mod openmct;
use dotenv::dotenv;
use influxdb2::Client;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct TelemetryServerState {
    influxdb_client: influxdb2::Client,
    realtime_channel: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let host = std::env::var("INFLUXDB_HOST").unwrap();
    let org = std::env::var("INFLUXDB_ORG").unwrap();
    let token = std::env::var("INFLUXDB_TOKEN").unwrap();
    let influxdb_client = Client::new(host, org, token);

    let (tx, _) = broadcast::channel(100);

    let state = TelemetryServerState {
        influxdb_client,
        realtime_channel: tx,
    };

    let app = Router::new()
        .nest("/openmct", openmct::routes::get_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
