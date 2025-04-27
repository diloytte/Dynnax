use serde::Deserialize;
use shared::types::SnipeConfig;

#[derive(Debug, Deserialize)]
pub struct CreateXSnipeTargetDTO {
    pub target_name: String,
    pub snipe_config: Option<SnipeConfig>,
    pub is_active: bool,
    pub deactivate_on_snipe: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PatchXSnipeTargetDTO {
    pub target_name: String,
    pub sol_amount: Option<f32>,
    pub slippage: Option<i32>,
    pub priority_fee: Option<f32>,
    pub is_active: Option<bool>,
    pub deactivate_on_snipe: Option<bool>,
}
