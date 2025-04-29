use std::sync::Arc;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response}, Extension,
};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use shared::json_error;
use crate::state::AppState;

pub async fn auth_middleware(
    authorization_token: TypedHeader<Authorization<Bearer>>,
    Extension(state): Extension<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let expected_token = &state.dynnax_api_key;

    let token = authorization_token.token();

    if token != expected_token {
        return (
            StatusCode::UNAUTHORIZED,
            json_error!("Unauthoirzed!")
        )
            .into_response();
    }

    next.run(req).await
}
