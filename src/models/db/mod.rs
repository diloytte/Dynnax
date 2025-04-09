use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DBSnipeTarget {
    pub id: i32,
    pub target_name: String,
    pub target_id: i64,
    pub sol_amount: f64,
    pub slippage: i32,
    pub priority_fee: f64,
    pub is_active: bool,
    pub deactivate_on_snipe: bool,
    pub past_shills: Vec<String>,
}
