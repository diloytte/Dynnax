use dashmap::DashMap;

use crate::types::SnipeTarget;
use grammers_client::{Client, Config, SignInError, Update};

pub struct AppState {
    pub channels: DashMap<i64, SnipeTarget>,
    pub tg_client: Option<Client>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            channels: DashMap::default(),
            tg_client: None,
        }
    }
}

impl AppState {
    pub fn set_tg_client(&mut self, client: Client) {
        self.tg_client = Some(client);
    }
}
