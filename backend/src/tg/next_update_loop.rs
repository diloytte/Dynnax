use std::sync::Arc;

use grammers_client::{Client, InvocationError, Update};
use token_address_extractor::{extract_all_solana_addresses, extract_solana_address};

use crate::{state::AppState, tg::sniper::snipe_x::snipe_x};

use super::sniper::snipe::snipe;

pub async fn main_tg_loop(
    client: Client,
    shared_state: Arc<AppState>,
    pf_api_key: String,
) -> Result<(), InvocationError> {
    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) => {
                let message_text = message.text();
                let chat_id = message.chat().id();

                let cas = extract_all_solana_addresses(message_text);

                if cas.is_empty() {
                    continue;
                }

                for ca in cas {
                    let client = client.clone();
                    let shared_state = shared_state.clone();
                    let message = message.clone();

                    tokio::spawn(async move {
                        if shared_state.redacted_custom_bot_id != chat_id {
                            let _ = snipe(chat_id, &client, &shared_state, &ca).await;
                        } else {
                            let _ = snipe_x(&message, &client, &shared_state, &ca).await;
                        }
                    });
                }
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => continue,
        }
    }
}
