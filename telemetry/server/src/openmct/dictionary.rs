use axum::{extract::Path, routing::get, Json, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/pods", get(pods))
        .route("/pods/:pod", get(get_pod))
        .route("/pods/:pod/measurements/:measurement", get(get_measurement))
}

async fn pods() -> Json<&'static [&'static str]> {
    // TODO: implement this
    Json(&["pod1", "pod2", "pod3"])
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
