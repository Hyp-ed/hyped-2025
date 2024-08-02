use crate::TelemetryServerState;
use axum::Router;

mod historical;
pub mod realtime;

pub fn get_routes() -> Router<TelemetryServerState> {
    Router::new()
        .nest("/historical", historical::get_routes())
        .nest("/realtime", realtime::get_routes())
}
