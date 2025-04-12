use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SnipeConfig {
    pub sol_amount: f32,
    pub slippage: i32,
    pub priority_fee: f32,
}

impl Default for SnipeConfig {
    fn default() -> Self {
        SnipeConfig {
            sol_amount: 1.0,
            slippage: 15,
            priority_fee: 0.005,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SnipeTarget {
    pub target_name: String,
    pub snipe_config: SnipeConfig,
    pub is_active: bool,
    pub deactivate_on_snipe: bool,
    pub past_shills: Vec<String>,
}

impl Default for SnipeTarget {
    fn default() -> Self {
        SnipeTarget {
            target_name: String::from("None"),
            snipe_config: SnipeConfig::default(),
            is_active: true,
            deactivate_on_snipe: true,
            past_shills: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TwitterTarget {
    pub snipe_config: SnipeConfig,
    pub is_active: bool,
    pub deactivate_on_snipe: bool,
}

impl Default for TwitterTarget {
    fn default() -> Self {
        Self {
            snipe_config: Default::default(),
            is_active: true,
            deactivate_on_snipe: true,
        }
    }
}
