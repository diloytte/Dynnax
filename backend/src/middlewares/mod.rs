use std::sync::Arc;

use crate::state::AppState;
use axum::{
    Extension,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use shared::json_error;

pub async fn auth_middleware(
    authorization_token: TypedHeader<Authorization<Bearer>>,
    Extension(state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let expected_token = &state.dynnax_api_key;

    let token = authorization_token.token();

    if token != expected_token {
        return (StatusCode::UNAUTHORIZED, json_error!("Unauthoirzed!")).into_response();
    }

    next.run(req).await
}
