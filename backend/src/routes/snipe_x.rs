use std::collections::HashMap;

use axum::{extract::Path, response::IntoResponse, routing::{delete, get, post}, Extension, Json, Router};
use serde_json::json;
use shared::{
    json_error,
    types::TwitterTarget,
};
use ureq::http::StatusCode;

use crate::{
    db::queries::x_snipe_targets::{q_create_x_snipe_target, q_delete_x_snipe_target},
    types::{dtos::snipe_x::CreateXSnipeTargetDTO, other::AppStateExtension},
};

pub fn routes() -> Router {
    Router::new().nest(
        "/snipeX",
        Router::new()
            .route("/", get(get_x_snipe_targets))
            .route("/", post(create_snipe_x_target))
            // .route("/", patch(patch_snipe_target))
            .route("/{id}", delete(delete_x_snipe_target)),
    )
}

async fn create_snipe_x_target(
    Extension(state): AppStateExtension,
    Json(twitter_create_snipe_dto): Json<CreateXSnipeTargetDTO>,
) -> impl IntoResponse {
    if state
        .twitter_snipe_targets
        .get(&twitter_create_snipe_dto.target_name).is_some()
    {
        return (
            StatusCode::BAD_REQUEST,
            json_error!(format!(
                "Twitter Snipe Target with Name: {} already exists.",
                &twitter_create_snipe_dto.target_name
            )),
        );
    }

    let twitter_target_name = &twitter_create_snipe_dto.target_name;
    let twitter_snipe_config_option = twitter_create_snipe_dto.snipe_config;
    let twitter_snipe_config = twitter_snipe_config_option.unwrap_or_default();

    if let Err(error) = q_create_x_snipe_target(
        &state.db,
        twitter_target_name,
        &twitter_snipe_config,
        &twitter_create_snipe_dto.deactivate_on_snipe,
    )
    .await
    {
        println!("Error <create_snipe_target>: {}", error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            json_error!("Something went wrong."),
        );
    }

    let twitter_snipe_targets = &state.twitter_snipe_targets;
    
    let twitter_snipe_target = TwitterTarget {
        target_name: twitter_target_name.clone(),
        snipe_config: twitter_snipe_config,
        is_active: true,
        deactivate_on_snipe: twitter_create_snipe_dto.deactivate_on_snipe.unwrap_or(true),
    };
    
    let response_data = json!({
        "snipe_target": twitter_snipe_target
    })
    .to_string();
    
    twitter_snipe_targets.insert(twitter_target_name.to_string(), twitter_snipe_target);
    
    (StatusCode::OK, response_data)
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

    let twitter_snipe_targets: &dashmap::DashMap<String, TwitterTarget> = &state.twitter_snipe_targets;
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
