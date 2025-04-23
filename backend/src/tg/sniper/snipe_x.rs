use crate::constants::GLOBALY_BLOCKED_CAS;
use crate::pf::buy_ca;
use crate::state::AppState;
use crate::twitter_regex::extract_twitter_sender;
use grammers_client::types::Message;
use grammers_client::{Client, InvocationError, InputMessage};
use shared::types::{Browser, SnipeTarget};
use shared::utils::{open_browser, play_buy_notif};
use token_address_extractor::extract_solana_address;
use std::sync::Arc;

pub async fn snipe_x(
    message: &Message,
    client: &Client,
    shared_state: &Arc<AppState>,
    ca: &String,
) -> Result<(), InvocationError> {
    let sender_option = extract_twitter_sender(message.text());
    
    if let None = sender_option {
        println!("Unable to find Twitter sender.");
        println!("Message:");
        println!("{}",message.text());
        println!("---------------");
        return Ok(());
    }
    
    let twitter_snipe_targets = &shared_state.twitter_snipe_targets;

    let twitter_sender = sender_option.unwrap();
    let twitter_snipe_target_option = twitter_snipe_targets.get(&twitter_sender);

    if let None = twitter_snipe_target_option {
        println!("{} is not a Twitter Snipe Target.",twitter_sender);
        return Ok(());
    }

    let twitter_snipe_target = twitter_snipe_target_option.unwrap();

    if !twitter_snipe_target.is_active{
        println!("{} is not a Active.",twitter_sender);
        return Ok(());
    }

    match buy_ca(&shared_state.pf_api_url,& twitter_snipe_target.snipe_config, &ca,1).await {
        Ok(_) => {
            println!("Triggered twitter sniper.");
            play_buy_notif();
            let chat_name = &twitter_snipe_target.target_name;
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
