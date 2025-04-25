use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub fn routes() -> Router {
    Router::new().route("/ping", get(pong))
}

async fn pong() -> impl IntoResponse {
    (StatusCode::OK, "pong")
}
