use serde::{Deserialize, Serialize};
use shared::types::{SnipeConfig, SnipeTarget, TwitterTarget};

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
    pub is_user: bool,
}

impl From<DBSnipeTarget> for SnipeTarget {
    fn from(db: DBSnipeTarget) -> Self {
        SnipeTarget {
            target_name: db.target_name,
            snipe_config: SnipeConfig {
                sol_amount: db.sol_amount as f32,
                slippage: db.slippage,
                priority_fee: db.priority_fee as f32,
            },
            is_active: db.is_active,
            deactivate_on_snipe: db.deactivate_on_snipe,
            past_shills: db.past_shills,
            is_user: db.is_user,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DBXSnipeTarget {
    pub id: i32,
    pub target_name: String,
    pub sol_amount: f64,
    pub slippage: i32,
    pub priority_fee: f64,
    pub is_active: bool,
    pub deactivate_on_snipe: bool,
}

impl From<DBXSnipeTarget> for TwitterTarget {
    fn from(db: DBXSnipeTarget) -> Self {
        TwitterTarget {
            target_name: db.target_name,
            snipe_config: SnipeConfig {
                sol_amount: db.sol_amount as f32,
                slippage: db.slippage,
                priority_fee: db.priority_fee as f32,
            },
            is_active: db.is_active,
            deactivate_on_snipe: db.deactivate_on_snipe,
            is_one_time: true, //TODO THIS, !!!
        }
    }
}
