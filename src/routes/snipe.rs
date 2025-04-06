use std::collections::HashMap;

use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use tokio::sync::RwLock;
use crate::models::{AppStateExtension, CreateSnipeDTO, SnipeConfig, SnipeTarget};

pub fn routes() -> Router {
    Router::new().nest(
        "/snipe",
        Router::new()
            .route("/", get(get_snipe_targets))
            .route("/", post(create_snipe_target)),
    )
}

async fn create_snipe_target(
    Extension(state): AppStateExtension,
    Json(create_snipe_dto): Json<CreateSnipeDTO>,
) -> impl IntoResponse {
    // TODO: make sure that target id is valid and exists
    let write_state = state.write().await;
    let dialogs = &write_state.snipe_targets;
    let snipe_target = SnipeTarget {
        target_name: create_snipe_dto.target_name,
        snipe_config: create_snipe_dto
            .snipe_config
            .unwrap_or(SnipeConfig::default()),
        is_active: false,
        deactivate_on_snipe: create_snipe_dto.deactivate_on_snipe.unwrap_or(true),
    };

    let response_data = json!({
        "snipe_target":snipe_target
    })
    .to_string();

    dialogs.insert(create_snipe_dto.target_id, snipe_target);

    (StatusCode::OK, response_data)
}

async fn get_snipe_targets(Extension(state): AppStateExtension) -> impl IntoResponse {
    let read_state = state.write().await;
    let snipe_targets = &read_state.snipe_targets;

    let mut snipe_targets_map: HashMap<i64, SnipeTarget> = HashMap::default();

    for entry in snipe_targets.iter() {
        let key = entry.key().clone();
        let value = entry.value().clone();
        snipe_targets_map.insert(key, value);
    }

    let dialogs_json_string = json!({
        "snipe_targets":snipe_targets_map
    })
    .to_string();

    (StatusCode::OK, dialogs_json_string)
}
