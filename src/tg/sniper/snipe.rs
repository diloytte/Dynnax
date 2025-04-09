use crate::{pf::buy_ca, state::AppState};
use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use token_address_extractor::extract_solana_address;

pub async fn snipe(
    client: Client,
    shared_state: Arc<AppState>,
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

                let snipe_targets = &shared_state.snipe_targets;
                let snipe_target_option = snipe_targets.get_mut(&chat_id);
                
                if let Some(mut snipe_target) = snipe_target_option {
                    
                    if !snipe_target.is_active {
                        continue;
                    }
                    match buy_ca(&pf_api_key, &snipe_target, ca.unwrap()).await {
                        
                        Ok(_) => {
                            snipe_target.deactivate_on_snipe = true;
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
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => {}
        }
    }
}
