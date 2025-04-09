use serde::Deserialize;

use crate::models::service::snipe_target::SnipeConfig;

#[derive(Debug, Deserialize)]
pub struct CreateSnipeDTO {
    pub target_name: String,
    pub target_id: i64,
    pub snipe_config: Option<SnipeConfig>,
    pub deactivate_on_snipe: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PatchSnipeTargetDTO {
    pub target_id: i64,
    pub target_name: Option<String>,
    pub sol_amount: Option<f32>,
    pub slippage: Option<i32>,
    pub priority_fee: Option<f32>,
    pub is_active: Option<bool>,
    pub deactive_on_snipe: Option<bool>,
}
