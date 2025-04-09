use axum::{Extension, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;
use tokio::sync::RwLock;

use crate::{
    models::other::AppStateExtension, state::AppState, tg::{self, dialog::clear_dialogs::clear_dialogs} 
};

pub fn routes() -> Router {
    Router::new().nest(
        "/tg",
        Router::new()
            .route("/", get(get_me))
            .route("/clear", get(clear_dialogs))
            .route("/dialogs", get(get_dialogs)),
    )
}

async fn get_me(Extension(state): AppStateExtension) -> impl IntoResponse {
    let tg_client = state.tg_client.as_ref().unwrap();
    let me_result = tg_client.get_me().await;

    if me_result.is_err() {
        println!(
            "ERROR: {}",
            me_result.err().unwrap()
        );
        return (StatusCode::INTERNAL_SERVER_ERROR,"Error getting me.".to_string());
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
    let client = state.tg_client.as_ref().unwrap();
    let dialogs_result = tg::dialog::get_dialogs::get_dialogs(client).await;
    let dialogs = dialogs_result.unwrap_or(vec![]);
    (
        StatusCode::OK,
        json!({
            "dialogs":dialogs
        })
        .to_string(),
    )
}
