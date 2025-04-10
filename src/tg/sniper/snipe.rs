use crate::{pf::buy_ca, state::AppState};
use grammers_client::types::Message;
use grammers_client::{Client, InvocationError, Update};
use std::sync::Arc;
use token_address_extractor::extract_solana_address;

pub async fn snipe(
    chat_id:i64,
    client: &Client,
    shared_state: &Arc<AppState>,
    pf_api_key: &String,
    ca:&String
) -> Result<(), InvocationError> {
    let snipe_targets = &shared_state.snipe_targets;
    let snipe_target_option = snipe_targets.get_mut(&chat_id);

    if let Some(mut snipe_target) = snipe_target_option {
        if !snipe_target.is_active {}
        match buy_ca(&pf_api_key, &snipe_target, ca.clone()).await {
            Ok(_) => {
                if snipe_target.deactivate_on_snipe {
                    snipe_target.is_active = false;
                }
                let chat_name = &snipe_target.target_name;
                let final_msg = format!(
                    "---------------\nChat: {}\n ID: {}\n CA: {}\n---------------",
                    chat_name,
                    chat_id,
                    ca
                );
                println!("{}", final_msg);
            }
            Err(error) => {
                println!("ERROR: {:?}", error)
            }
        }
    }
    Ok(())
}
