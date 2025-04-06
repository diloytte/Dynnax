use crate::{pf::buy_ca, state::AppState};
use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use token_address_extractor::{extract_solana_address, extract_token_address_from_message_text};
use tokio::sync::RwLock;

pub async fn listen_for_updates(
    client: Client,
    shared_state: Arc<RwLock<AppState>>,
    pf_api_key: String,
) -> Result<(), InvocationError> {
    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) => {
                let chat_id = message.chat().id();
                let ca = extract_solana_address(message.text());
                if ca.is_none() {
                    continue;
                }
                dbg!(&ca);

                {
                    let read_state = shared_state.read().await;
                    let chats = &read_state.dialogs;
                    let snipe_target_option = chats.get_mut(&chat_id);
                    if let Some(mut snipe_target) = snipe_target_option {
                        if !snipe_target.is_active {
                            continue;
                        }

                        match buy_ca(&pf_api_key, &snipe_target, ca.unwrap()).await {
                            Ok(_) => {
                                snipe_target.set_deactivate_on_snipe();
                            }
                            Err(error) => {
                                println!("ERROR: {}", error)
                            }
                        }
                    } else {
                        let chat = message.chat();
                        let chat_name = chat.name();
                        println!(
                            "Cannot find chat id: {} with name: {} in snipe targets map.",
                            chat_id, chat_name
                        );
                    };
                }

                let ca_option = extract_token_address_from_message_text(message.text());
                let ca = ca_option.unwrap_or(String::from("None"));
                println!("Solana token not found. Extracted CA: {}", ca);
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }
}
