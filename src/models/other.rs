use std::sync::Arc;

use axum::Extension;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug,Serialize)]
pub struct TradeRequest {
    pub action: String,
    pub mint: String,
    pub amount: f32,
    #[serde(rename = "denominatedInSol")]
    pub denominated_in_sol: String,
    pub slippage: i32,
    #[serde(rename = "priorityFee")]
    pub priority_fee: f32,
    pub pool: String,
}

pub type AppStateExtension = Extension<Arc<AppState>>;

pub enum Browser {
    Brave,
}