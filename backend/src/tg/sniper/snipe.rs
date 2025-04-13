use crate::{models::other::Browser, pf::buy_ca, state::AppState, utils::open_browser};
use grammers_client::{Client, InputMessage, InvocationError};
use std::sync::Arc;

pub async fn snipe(
    chat_id: i64,
    client: &Client,
    shared_state: &Arc<AppState>,
    pf_api_key: &String,
    ca: &String,
) -> Result<(), InvocationError> {
    let snipe_targets = &shared_state.snipe_targets;
    let snipe_target_option = snipe_targets.get_mut(&chat_id);

    if let Some(mut snipe_target) = snipe_target_option {
        match buy_ca(pf_api_key, &snipe_target, ca.clone()).await {
            Ok(_) => {
                if snipe_target.deactivate_on_snipe {
                    snipe_target.is_active = false;
                }
                let chat_name = &snipe_target.target_name;
                let final_msg = format!(
                    "---------------\nChat: {}\n ID: {}\n CA: {}\n---------------",
                    chat_name, chat_id, ca
                );
                println!("{}", final_msg);
                let bullx_link = format!(
                    "https://neo.bullx.io/terminal?chainId=1399811149&address={}",
                    ca
                );
                let _ = open_browser(
                    Browser::Brave,
                    &bullx_link,
                );
                let trenches_chat = &shared_state.sniper_trenches_chat;
                client.send_message(trenches_chat,InputMessage::text(format!("{}\n{}",final_msg,bullx_link))).await?;
            }
            Err(error) => {
                println!("ERROR: {:?}", error)
            }
        }
    }
    Ok(())
}
