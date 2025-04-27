use std::fmt::Display;

use grammers_client::{client, types::Chat, Client, InputMessage, InvocationError};
use shared::{types::Browser, utils::{open_browser, play_buy_notif}};

pub mod snipe;
pub mod snipe_x;

pub enum Shiller {
    Tg(i64),
    X(String)
}

impl Display for Shiller{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Shiller::Tg(chat_id) => write!(f, "{}", chat_id),
            Shiller::X(account) => write!(f, "{}", account),
        }
    }
}

pub async fn buy_notify<'a>(chat_name:&str,shiller:&Shiller,ca:&str,client:&Client,trenches_chat:&Chat,trojan_bot:&Chat)->Result<(),InvocationError>{
    play_buy_notif();
    let final_msg = format!(
        "---------------\nChat: {}\n ID: {}\n CA: {}\n---------------",
        chat_name, shiller, ca
    );
    let bullx_link = format!(
        "https://neo.bullx.io/terminal?chainId=1399811149&address={}",
        ca
    );
    let _ = open_browser(
        Browser::Brave,
        &bullx_link,
    );
    {
        let client = client.clone();
        let trenches_chat = trenches_chat.clone();
        let trojan_bot = trojan_bot.clone();
        let final_msg = final_msg.clone();
        let bullx_link = bullx_link.clone();
        tokio::spawn(async move {
            let _ = client.send_message(trenches_chat,InputMessage::text(format!("{}\n{}",final_msg,bullx_link))).await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let _ = client.send_message(trojan_bot, "/positions").await;
        });
    }
    Ok(())
}
