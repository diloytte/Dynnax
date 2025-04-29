use axum::{Extension, Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use serde::Deserialize;
use shared::{json_error, pf::manual_buy_token};

use crate::types::other::AppStateExtension;

#[derive(Deserialize)]
pub struct ManualBuyDTO {
    ca: String,
    sol_amount: f32,
}

pub fn routes() -> Router {
    Router::new().nest("/pf", Router::new().route("/buy", post(manual_buy)))
}

async fn manual_buy(
    Extension(state): AppStateExtension,
    Json(manual_buy_dto): Json<ManualBuyDTO>,
) -> impl IntoResponse {
    match manual_buy_token(
        &state.pf_api_url,
        manual_buy_dto.sol_amount,
        manual_buy_dto.ca,
        &state.request_client,
    )
    .await
    {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(error) => {
            println!("{:?}", error);
            (StatusCode::OK, json_error!("Something went wrong.")).into_response()
        }
    }
}

// async fn manual_sell(
//     Extension(state):AppStateExtension,
//     Json(manual_sell_dto):Json<ManualBuyDTO>
// ) -> impl IntoResponse{
//     sell_ca(&state.pf_api_url, snipe_target, ca, sell_precent)
// }
