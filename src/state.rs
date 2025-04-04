use dashmap::DashMap;

use crate::types::SnipeTarget;
use grammers_client::Client;

#[derive(Default)]
pub struct AppState {
    pub channels: DashMap<i64, SnipeTarget>,
    pub tg_client: Option<Client>,
}


impl AppState {
    pub fn set_tg_client(&mut self, client: Client) {
        self.tg_client = Some(client);
    }
}
