use grammers_client::types::Chat;

use dashmap::DashMap;

use grammers_client::Client;
use shared::types::{SnipeTarget, TwitterTarget};

use crate::db::connect::Database;

pub struct AppState {
    pub db: Database,
    pub all_dialogs: DashMap<i64, (String, u8)>,
    pub snipe_targets: DashMap<i64, SnipeTarget>,
    pub twitter_snipe_targets: DashMap<String, TwitterTarget>,
    pub tg_client: Option<Client>,
    pub redacted_custom_bot_id: i64,
    pub sniper_trenches_chat:Chat,
    pub pf_api_url:String,
    pub priority_fee_multiplier:u8 //Important: PumpFun Portal API dev said that fee is split for 3 sections. Jito/Fee/Priority Fee. Hence sending more makes tx land faster.
}
