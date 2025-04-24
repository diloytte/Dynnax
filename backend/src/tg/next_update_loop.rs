use std::sync::Arc;

use grammers_client::{Client, InvocationError, Update};
use token_address_extractor::extract_solana_address;

use crate::{sniper::{snipe::snipe, snipe_x::snipe_x}, state::AppState};


pub async fn main_tg_loop(
    client: Client,
    shared_state: Arc<AppState>,
) -> Result<(), InvocationError> {
    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) => {
                let message_text = message.text();
                let chat_id = message.chat().id();

                let ca = extract_solana_address(message_text);

                if ca.is_none() {
                    continue;
                }

                let ca = ca.unwrap();

                let client = client.clone();
                let shared_state = shared_state.clone();
                let message = message.clone();

                if shared_state.redacted_custom_bot_id != chat_id {
                    tokio::spawn(async move {
                        let _ = snipe(chat_id, &client, &shared_state, &ca).await;
                    });
                } else {
                    tokio::spawn(async move {
                        let _ = snipe_x(&message, &client, &shared_state, &ca).await;
                    });
                }
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => continue,
        }
    }
}
