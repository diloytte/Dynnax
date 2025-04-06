use dashmap::DashMap;

use grammers_client::Client;

use crate::models::SnipeTarget;

#[derive(Default)]
pub struct AppState {
    pub snipe_targets: DashMap<i64, SnipeTarget>,
    pub tg_client: Option<Client>,
}

impl AppState {
    pub fn set_tg_client(&mut self, client: Client) {
        self.tg_client = Some(client);
    }
}
