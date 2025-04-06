
use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use serde_json::json;
use tokio::sync::RwLock;

use crate::models::{AppStateExtension, CreateSnipeDTO};
async fn create_snipe(Extension(state): AppStateExtension,Json(create_snipe_dto):Json<CreateSnipeDTO>) -> impl IntoResponse {

}