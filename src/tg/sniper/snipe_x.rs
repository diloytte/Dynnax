use crate::state::AppState;
use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn _snipe_x(
    client: Client,
    _shared_state: Arc<RwLock<AppState>>,
    _pf_api_key: String,
) -> Result<(), InvocationError> {
    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) => {
                let _chat_id = message.chat().id();
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }
}
