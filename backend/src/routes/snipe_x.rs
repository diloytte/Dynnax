use std::collections::HashMap;

use crate::{
    db::queries::x_snipe_targets::{
        q_create_x_snipe_target, q_delete_x_snipe_target, q_patch_x_snipe_target,
    },
    types::{
        dtos::snipe_x::{CreateXSnipeTargetDTO, PatchXSnipeTargetDTO},
        other::AppStateExtension,
    },
};
use axum::{
    Extension, Json, Router,
    extract::Path,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use reqwest::StatusCode;
use serde_json::json;
use shared::{json_error, types::TwitterTarget};

use super::snipe_x_internal::create_snipe_x_target_internal;

pub fn routes() -> Router {
    Router::new().nest(
        "/snipeX",
        Router::new()
            .route("/", get(get_x_snipe_targets))
            .route("/", post(create_snipe_x_target))
            .route("/bulk", post(create_bulk_snipe_x_targets))
            .route("/", patch(patch_x_snipe_target))
            .route("/{id}", delete(delete_x_snipe_target)),
    )
}


async fn create_bulk_snipe_x_targets(
    Extension(state): AppStateExtension,
    Json(dtos): Json<Vec<CreateXSnipeTargetDTO>>,
) -> impl IntoResponse {
    let mut created = Vec::new();
    let mut failed = Vec::new();

    for dto in dtos {
        match create_snipe_x_target_internal(&state, dto).await {
            Ok(twitter_snipe_target) => {
                created.push(twitter_snipe_target);
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


async fn create_snipe_x_target(
    Extension(state): AppStateExtension,
    Json(dto): Json<CreateXSnipeTargetDTO>,
) -> impl IntoResponse {
    match create_snipe_x_target_internal(&state, dto).await {
        Ok(twitter_snipe_target) => {
            let response_data = json!({
                "snipe_target": twitter_snipe_target
            })
            .to_string();
            (StatusCode::OK, response_data)
        }
        Err(message) => {
            let status = if message.contains("already exists") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, json_error!(message))
        }
    }
}


async fn get_x_snipe_targets(Extension(state): AppStateExtension) -> impl IntoResponse {
    let twitter_snipe_targets = &state.twitter_snipe_targets;

    let mut snipe_targets_map: HashMap<String, TwitterTarget> = HashMap::default();

    for entry in twitter_snipe_targets.iter() {
        let key = entry.key();
        let value = entry.value().clone();
        snipe_targets_map.insert(key.to_string(), value);
    }

    let dialogs_json_string = json!({
        "twitter_snipe_targets":snipe_targets_map
    })
    .to_string();

    (StatusCode::OK, dialogs_json_string)
}

async fn delete_x_snipe_target(
    Extension(state): AppStateExtension,
    Path(target_name): Path<String>,
) -> impl IntoResponse {
    if let Err(error) = q_delete_x_snipe_target(&state.db, &target_name).await {
        println!("Error: <delete_snipe_target>: {}", error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json_error!("Something went wrong"),
        );
    }

    let twitter_snipe_targets: &dashmap::DashMap<String, TwitterTarget> =
        &state.twitter_snipe_targets;
    let removed_target = twitter_snipe_targets.remove(&target_name);

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
                &target_name
            )),
        ),
    }
}

async fn patch_x_snipe_target(
    Extension(state): AppStateExtension,
    Json(patch_x_snipe_target_dto): Json<PatchXSnipeTargetDTO>,
) -> impl IntoResponse {
    if let Err(err) = q_patch_x_snipe_target(&state.db, &patch_x_snipe_target_dto).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json_error!(format!("Failed to patch DB: {}", err)),
        );
    }

    let twitter_snipe_targets: &dashmap::DashMap<String, TwitterTarget> =
        &state.twitter_snipe_targets;
    let snipe_target_option = twitter_snipe_targets.get_mut(&patch_x_snipe_target_dto.target_name);

    if let Some(mut snipe_target) = snipe_target_option {
        if let Some(sol) = patch_x_snipe_target_dto.sol_amount {
            snipe_target.snipe_config.sol_amount = sol;
        }

        if let Some(slippage) = patch_x_snipe_target_dto.slippage {
            snipe_target.snipe_config.slippage = slippage;
        }

        if let Some(priority_fee) = patch_x_snipe_target_dto.priority_fee {
            snipe_target.snipe_config.priority_fee = priority_fee;
        }

        if let Some(is_active) = patch_x_snipe_target_dto.is_active {
            snipe_target.is_active = is_active;
        }

        if let Some(deactivate_on_snipe) = patch_x_snipe_target_dto.deactivate_on_snipe {
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
            &patch_x_snipe_target_dto.target_name
        )),
    )
}
