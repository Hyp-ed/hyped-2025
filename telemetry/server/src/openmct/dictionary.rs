use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use hyped_config;

use crate::TelemetryServerState;

pub fn get_routes() -> Router<TelemetryServerState> {
    Router::new()
        .route("/pods", get(pods))
        .route("/pods/:pod", get(get_pod))
        .route("/pods/:pod/measurements/:measurement", get(get_measurement))
}

async fn pods() -> impl IntoResponse {
    println!("Hello, world!");
    let pod_config = hyped_config::get_pod_config();
    println!("{:?}", pod_config);
    Json(pod_config.pod_ids)
}

async fn get_pod(Path(pod): Path<String>) -> Json<&'static str> {
    // TODO: implement this
    Json(match pod.as_str() {
        "pod1" => "pod1",
        "pod2" => "pod2",
        "pod3" => "pod3",
        _ => "not found",
    })
}

async fn get_measurement(Path((_pod, _measurement)): Path<(String, String)>) -> Json<&'static str> {
    // TODO: implement this
    Json("measurement")
}
