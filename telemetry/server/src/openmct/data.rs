use crate::TelemetryServerState;
use axum::Router;

mod historical;

pub fn get_routes() -> Router<TelemetryServerState> {
    Router::new().nest("/historical", historical::get_routes())
}
