use std::os::unix;
use grammers_client::types::Chat;

use dashmap::DashMap;

use grammers_client::Client;

use crate::{
    db::connect::Database,
    models::service::snipe_target::{SnipeTarget, TwitterTarget},
};

pub struct AppState {
    pub db: Database,
    pub all_dialogs: DashMap<i64, (String, u8)>,
    pub snipe_targets: DashMap<i64, SnipeTarget>,
    pub twitter_snipe_targets: DashMap<String, TwitterTarget>,
    pub tg_client: Option<Client>,
    pub redacted_custom_bot_id: i64,
    pub sniper_trenches_chat:Chat
}
