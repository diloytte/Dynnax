use std::sync::Arc;

use axum::Extension;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Serialize)]
pub struct TradeRequest {
    pub action: String,
    pub mint: String,
    pub amount: f32,
    pub denominated_in_sol: bool,
    pub slippage: i32,
    pub priority_fee: f32,
    pub pool: String,
}
#[derive(Debug, Deserialize)]
pub struct MyRecvBody {
    pub _other: String,
}

pub type AppStateExtension = Extension<Arc<AppState>>;
