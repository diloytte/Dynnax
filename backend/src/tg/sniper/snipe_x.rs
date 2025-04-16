use crate::models::other::Browser;
use crate::models::service::snipe_target::SnipeTarget;
use crate::pf::buy_ca;
use crate::state::AppState;
use crate::utils::{open_browser, play_buy_notif};
use grammers_client::types::Message;
use grammers_client::{Client, InvocationError, InputMessage};
use token_address_extractor::extract_solana_address;
use std::sync::Arc;

pub async fn snipe_x(
    message: &Message,
    client: &Client,
    shared_state: &Arc<AppState>,
    ca: &String,
) -> Result<(), InvocationError> {
    let blocked_ca = "43SXvpf4c41t2uErsw7aL6w5qhnie6BXSSPqiTcTpump";

    if ca.to_lowercase() == blocked_ca.to_lowercase() {
        return Ok(());
    }

    let mut snipe_target = SnipeTarget::default();

    let sol_amount = if cfg!(feature = "production"){
        2.5
    } else {
        0.0001
    };

    snipe_target.snipe_config.sol_amount=sol_amount;

    match buy_ca(&shared_state.pf_api_url,& snipe_target, &ca,1).await {
        Ok(_) => {
            play_buy_notif();
            let chat_name = &snipe_target.target_name;
            let final_msg = format!(
                "---------------\nChat: {}\n CA: {}\n---------------",
                chat_name, ca
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
        },
        Err(error) => {
            println!("{:?}",error);
        },
    }

    Ok(())
}
