use std::sync::Arc;

use grammers_client::{Client, InvocationError, Update};
use token_address_extractor::extract_solana_address;

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

                let ca = extract_solana_address(message_text);
                if ca.is_none() {
                    continue;
                }

                let chat_id = message.chat().id();
                if shared_state.redacted_custom_bot_id != chat_id {
                    let snipe_result = snipe(
                        chat_id,
                        &client,
                        &shared_state,
                        &pf_api_key,
                        ca.as_ref().unwrap(),
                    )
                    .await;
                } else {
                    let snipe_x_result = snipe_x(
                        &message,
                        &client,
                        &shared_state,
                        &pf_api_key,
                        ca.as_ref().unwrap(),
                    )
                    .await;
                }
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }
}
