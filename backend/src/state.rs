use reqwest::Client as ReqwestClient;
use std::sync::Arc;

use grammers_client::types::Chat;

use dashmap::DashMap;

use grammers_client::Client;
use shared::{
    db::Database,
    types::{SnipeTarget, TwitterTarget},
};

pub struct AppState {
    pub dynnax_api_key: String,
    pub request_client: ReqwestClient,
    pub db: Database,
    pub all_dialogs: DashMap<i64, (String, u8)>,
    pub snipe_targets: DashMap<i64, SnipeTarget>,
    pub twitter_snipe_targets: DashMap<String, TwitterTarget>,
    pub tg_client: Client,
    pub tg_client_informer: Client,
    pub redacted_custom_bot_id: i64,
    pub redacted_bot_chat: Chat,
    pub sniper_trenches_chat: Arc<Chat>,
    pub trojan_bot_chat: Arc<Chat>,
    pub pf_api_url: String,
    pub shill_groups: Vec<Chat>,
    pub priority_fee_multiplier: u8, //Important: PumpFun Portal API dev said that fee is split for 3 sections. Jito/Fee/Priority Fee. Hence sending more makes tx land faster.
}
