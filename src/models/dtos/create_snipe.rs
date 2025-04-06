use serde::Deserialize;

use crate::models::SnipeConfig;

#[derive(Debug, Deserialize)]
pub struct CreateSnipeDTO {
    pub target_name: String,
    pub target_id: String,
    pub snipe_config: Option<SnipeConfig>,
    pub deactivate_on_snipe: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSnipeConfigDTO{
    pub sol_amount: Option<f32>,
    pub slippage: Option<u8>,
    pub priority_fee: Option<f32>,
}


