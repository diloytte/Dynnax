use crate::{pf::buy_ca, state::AppState};
use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use token_address_extractor::extract_solana_address;
use tokio::sync::RwLock;

pub async fn listen_for_updates(
    client: Client,
    shared_state: Arc<RwLock<AppState>>,
) -> Result<(), InvocationError> {
    let read_state = shared_state.read().await;
    let chats = &read_state.chats;
   
    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) => {
                let chat_id = message.chat().id();
                let snipe_target_option = chats.get(&chat_id);
                if let Some(snipe_target) = snipe_target_option {
                    let _ = buy_ca("", &snipe_target).await;
                };

                let ca = extract_solana_address(message.text());
                dbg!(ca);
                dbg!(message.text());

                //check if it contains any ca at all and inform
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }
}
