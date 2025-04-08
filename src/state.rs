use dashmap::DashMap;

use grammers_client::Client;

use crate::models::{SnipeTarget, TwitterTarget};

#[derive(Default)]
pub struct AppState {
    pub snipe_targets: DashMap<i64, SnipeTarget>,
    pub twitter_snipe_targets: DashMap<String,TwitterTarget>,
    pub tg_client: Option<Client>,
}