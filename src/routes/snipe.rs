use std::collections::HashMap;

use crate::models::{
    AppStateExtension, CreateSnipeDTO, PatchSnipeTargetDTO, SnipeConfig, SnipeTarget,
};
use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use grammers_client::grammers_tl_types::types::smsjobs::Status;
use serde_json::json;
use tokio::sync::RwLock;

pub fn routes() -> Router {
    Router::new().nest(
        "/snipe",
        Router::new()
            .route("/", get(get_snipe_targets))
            .route("/", post(create_snipe_target))
            .route("/", patch(patch_snipe_target))
            .route("/{id}", delete(delete_snipe_target)),
    )
}

async fn create_snipe_target(
    Extension(state): AppStateExtension,
    Json(create_snipe_dto): Json<CreateSnipeDTO>,
) -> impl IntoResponse {
    // TODO: make sure that target id is valid and exists

    let dialogs = &state.snipe_targets;
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
    let snipe_targets = &state.snipe_targets;

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

async fn patch_snipe_target(
    Extension(state): AppStateExtension,
    Json(patch_snipe_target_dto): Json<PatchSnipeTargetDTO>,
) -> impl IntoResponse {
    let snipe_targets: &dashmap::DashMap<i64, SnipeTarget> = &state.snipe_targets;
    let snipe_target_option = snipe_targets.get_mut(&patch_snipe_target_dto.target_id);

    if let Some(mut snipe_target) = snipe_target_option {
        if let Some(name) = patch_snipe_target_dto.target_name {
            snipe_target.target_name = name;
        }

        if let Some(sol) = patch_snipe_target_dto.sol_amount {
            snipe_target.snipe_config.sol_amount = sol;
        }

        if let Some(slippage) = patch_snipe_target_dto.slippage {
            snipe_target.snipe_config.slippage = slippage;
        }

        if let Some(priority_fee) = patch_snipe_target_dto.priority_fee {
            snipe_target.snipe_config.priority_fee = priority_fee;
        }

        if let Some(is_active) = patch_snipe_target_dto.is_active {
            snipe_target.is_active = is_active;
        }

        if let Some(deactivate_on_snipe) = patch_snipe_target_dto.deactive_on_snipe {
            snipe_target.deactivate_on_snipe = deactivate_on_snipe;
        }

        let snipe_target = snipe_target.to_owned();

        let data_response = Json(json!({
            "snipe_target":snipe_target
        }));

        return (StatusCode::OK, data_response);
    }

    let error_response = Json(json!({
        "error":format!("Snipe target with ID: {} does not exist.",&patch_snipe_target_dto.target_id)
    }));

    (StatusCode::NOT_FOUND, error_response)
}

async fn delete_snipe_target(
    Extension(state): AppStateExtension,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let snipe_targets: &dashmap::DashMap<i64, SnipeTarget> = &state.snipe_targets;
    let remove = snipe_targets.remove(&id);

    match remove {
        Some(snipe_target) => {
            return (
                StatusCode::OK,
                Json(json!({
                    "snipe_target":snipe_target
                })),
            );
        }
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error":format!("Snipe Target with ID: {} does not exist.",&id)})),
            );
        }
    }
}
