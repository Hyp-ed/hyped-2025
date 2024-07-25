use axum::{routing::get, Json, Router};

use crate::{
    openmct::{
        data, dictionary,
        object_types::{OpenMctObject, OPEN_MCT_OBJECT_TYPES},
    },
    TelemetryServerState,
};

pub fn get_routes() -> Router<TelemetryServerState> {
    Router::new()
        .route("/", get(handler))
        .route("/object-types", get(get_object_types))
        .nest("/dictionary", dictionary::get_routes())
        .nest("/data", data::get_routes())
}

async fn handler() -> String {
    "Hello, Open MCT!".to_string()
}

async fn get_object_types() -> Json<&'static [OpenMctObject]> {
    Json(OPEN_MCT_OBJECT_TYPES)
}
