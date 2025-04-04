use std::sync::Arc;

use axum::{Extension, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;
use tokio::sync::RwLock;

use crate::{constants::INTERNAL_ERROR_CODES, state::AppState};

pub fn routes() -> Router {
    Router::new().nest("/tg", Router::new().route("/me", get(get_me)))
}

async fn get_me(Extension(state): Extension<Arc<RwLock<AppState>>>) -> impl IntoResponse {
    let locked_read_state = state.read().await;
    let tg_client = locked_read_state.tg_client.as_ref().unwrap();
    let me_result = tg_client.get_me().await;

    let get_me_code_error = INTERNAL_ERROR_CODES.get("GETME").unwrap();

    if me_result.is_err() {
        println!(
            "CODE: {}\nERROR: {}",
            get_me_code_error,
            me_result.err().unwrap()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong. CODE: {}", get_me_code_error),
        );
    }

    let me = me_result.unwrap();

    (
        StatusCode::OK,
        json!({
            "first_name":me.first_name(),
            "last_name":me.last_name(),
            "username":me.username(),
            "is_verified":me.verified()
        })
        .to_string(),
    )
}
