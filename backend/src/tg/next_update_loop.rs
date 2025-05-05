use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use token_address_extractor::extract_solana_address;

use crate::{
    sniper::{snipe::snipe, snipe_x::snipe_x},
    state::AppState,
};

pub async fn main_tg_loop(
    client: Client,
    shared_state: Arc<AppState>,
) -> Result<(), InvocationError> {
    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) => {
                let message_text = message.text();
                let ca = extract_solana_address(message_text);

                if ca.is_none() {
                    continue;
                }

                let message_sender_id = message.sender().unwrap().id();
                let chat_id = message.chat().id();
                let ca = ca.unwrap();


                if shared_state.redacted_custom_bot_id != chat_id {
                    let _ = snipe(chat_id,message_sender_id, &client, &shared_state, &ca).await;
                } else {
                    let _ = snipe_x(&message, &client, &shared_state, &ca).await;
                }
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => continue,
        }
    }
}
