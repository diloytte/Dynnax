use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router
};
use serde_json::json;
use shared::{
    json_error,
    tg::dialog::{
        clear_dialogs::clear_dialogs,
        get_dialogs::{SimplifiedDialog, get_dialogs as tg_all_dialogs, simplify_dialog},
    },
};

use crate::{tg::shill::shill_in_groupchats, types::{dtos::tg::{IgnoreUserDTO, ShillDTO}, other::AppStateExtension}};

pub fn routes() -> Router {
    Router::new().nest(
        "/tg",
        Router::new()
            .route("/", get(get_me))
            .route("/clear", post(clear_dialogs_route))
            .route("/dialogs", get(get_dialogs))
            .route("/shill", post(shill)),
    )
}

async fn shill(Extension(state): AppStateExtension,Json(shill_dto):Json<ShillDTO>) -> impl IntoResponse {
    let shill_result =
        shill_in_groupchats(&state.shill_groups, &state.tg_client,&shill_dto.shill_message,shill_dto.is_test,&state.sniper_trenches_chat).await;
    if shill_result.is_err() {
        println!("Error: {:?}", shill_result.err());
        return (StatusCode::CONFLICT, json_error!("Error in shilling."));
    }

    (StatusCode::OK, "".to_string())
}
async fn clear_dialogs_route(Extension(state): AppStateExtension, Json(ignore_user_dto):Json<IgnoreUserDTO>) -> impl IntoResponse {
    match clear_dialogs(&state.tg_client, if ignore_user_dto.ignore_user.is_none() {Some(true)} else {Some(false)}).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(error) => (StatusCode::OK, json_error!(error.to_string())).into_response(),
    };
}

async fn get_me(Extension(state): AppStateExtension) -> impl IntoResponse {
    let tg_client = &state.tg_client;
    let me_result = tg_client.get_me().await;

    if me_result.is_err() {
        println!("ERROR: {}", me_result.err().unwrap());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error getting me.".to_string(),
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

async fn get_dialogs(Extension(state): AppStateExtension) -> impl IntoResponse {
    let client = &state.tg_client;
    let dialogs_result = tg_all_dialogs(client).await;
    let dialogs = dialogs_result.unwrap_or(vec![]);
    let dialogs_simplified: Vec<SimplifiedDialog> = dialogs
        .iter()
        .map(|dialog| simplify_dialog(dialog))
        .collect();
    (
        StatusCode::OK,
        json!({
            "dialogs":dialogs_simplified
        })
        .to_string(),
    )
}
