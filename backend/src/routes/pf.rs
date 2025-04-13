use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::{json_error, models::other::AppStateExtension, pf::manual_buy_token};

#[derive(Deserialize)]
pub struct ManualBuyDTO{
    ca:String,
    sol_amount:f32
}

pub fn routes()->Router{
    Router::new().nest(
        "/pf",
        Router::new()
        .route("/buy", post(manual_buy))
    )
}


async fn manual_buy(
    Extension(state):AppStateExtension,
    Json(manual_buy_dto):Json<ManualBuyDTO>
)-> impl IntoResponse{
    match manual_buy_token(&state.pf_api_url, manual_buy_dto.sol_amount,manual_buy_dto.ca).await{
        Ok(_) => {
            return (StatusCode::OK).into_response();
        },
        Err(error) =>{
            println!("{:?}",error);
            return (StatusCode::OK,json_error!("Something went wrong.")).into_response();
        },
    }
}