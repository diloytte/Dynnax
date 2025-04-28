use std::collections::HashMap;

use crate::{
    db::queries::snipe_targets::{
        q_create_snipe_target, q_delete_snipe_target, q_patch_snipe_target,
    },
    types::{
        dtos::{CreateSnipeDTO, PatchSnipeTargetDTO},
        other::AppStateExtension,
    },
};
use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use serde_json::json;
use shared::{
    json_error,
    types::{SnipeConfig, SnipeTarget},
};

use super::snipe_internal::create_snipe_target_internal;

pub fn routes() -> Router {
    Router::new().nest(
        "/snipe",
        Router::new()
            .route("/", get(get_snipe_targets))
            .route("/", post(create_snipe_target))
            .route("/bulk", post(create_bulk_snipe_targets))
            .route("/", patch(patch_snipe_target))
            .route("/{id}", delete(delete_snipe_target)),
    )
}

async fn create_bulk_snipe_targets(
    Extension(state): AppStateExtension,
    Json(create_snipe_dtos): Json<Vec<CreateSnipeDTO>>,
) -> impl IntoResponse {
    let mut created = Vec::new();
    let mut failed = Vec::new();

    for dto in create_snipe_dtos {
        match create_snipe_target_internal(&state, dto).await {
            Ok(snipe_target) => {
                created.push(snipe_target);
            }
            Err(error_message) => {
                failed.push(error_message);
            }
        }
    }

    let response_data = json!({
        "created": created,
        "failed": failed
    })
    .to_string();

    (StatusCode::OK, response_data)
}



async fn create_snipe_target(
    Extension(state): AppStateExtension,
    Json(create_snipe_dto): Json<CreateSnipeDTO>,
) -> impl IntoResponse {
    match create_snipe_target_internal(&state, create_snipe_dto).await {
        Ok(snipe_target) => {
            let response_data = json!({
                "snipe_target": snipe_target
            })
            .to_string();
            (StatusCode::OK, response_data)
        }
        Err(message) => {
            let status = if message.contains("does not exist") {
                StatusCode::NOT_FOUND
            } else if message.contains("already exists") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            (status, json_error!(message))
        }
    }
}


async fn get_snipe_targets(Extension(state): AppStateExtension) -> impl IntoResponse {
    let snipe_targets = &state.snipe_targets;

    let mut snipe_targets_map: HashMap<i64, SnipeTarget> = HashMap::default();

    for entry in snipe_targets.iter() {
        let key = *entry.key();
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
    if let Err(err) = q_patch_snipe_target(&state.db, &patch_snipe_target_dto).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json_error!(format!("Failed to patch DB: {}", err)),
        );
    }

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

        return (StatusCode::OK, data_response.to_string());
    }

    (
        StatusCode::NOT_FOUND,
        json_error!(format!(
            "Snipe target with ID: {} does not exist.",
            &patch_snipe_target_dto.target_id
        )),
    )
}

async fn delete_snipe_target(
    Extension(state): AppStateExtension,
    Path(target_id): Path<i64>,
) -> impl IntoResponse {
    if let Err(error) = q_delete_snipe_target(&state.db, target_id).await {
        println!("Error: <delete_snipe_target>: {}", error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json_error!("Something went wrong"),
        );
    }

    let snipe_targets: &dashmap::DashMap<i64, SnipeTarget> = &state.snipe_targets;
    let removed_target = snipe_targets.remove(&target_id);

    match removed_target {
        Some(snipe_target) => (
            StatusCode::OK,
            json!({
                "snipe_target":snipe_target
            })
            .to_string(),
        ),
        None => (
            StatusCode::NOT_FOUND,
            json_error!(format!(
                "Snipe Target with ID: {} does not exist.",
                &target_id
            )),
        ),
    }
}
