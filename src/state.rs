use dashmap::DashMap;

use grammers_client::Client;

use crate::{db::connect::Database, models::service::snipe_target::{SnipeTarget, TwitterTarget}};

pub struct AppState {
    pub snipe_targets: DashMap<i64, SnipeTarget>,
    pub twitter_snipe_targets: DashMap<String,TwitterTarget>,
    pub tg_client: Option<Client>,
    pub db:Database
}