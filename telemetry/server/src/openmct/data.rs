use axum::Router;

mod historical;

pub fn get_routes() -> Router {
    Router::new().nest("/historical", historical::get_routes())
}
