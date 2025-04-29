use std::fmt::Display;

use grammers_client::{Client, InputMessage, InvocationError, types::Chat};

#[cfg(not(feature = "remote"))]
use shared::types::Browser;

#[cfg(not(feature = "remote"))]
use shared::utils::open_browser;

pub mod snipe;
pub mod snipe_x;

pub enum Shiller {
    Tg(i64),
    X(String),
}

impl Display for Shiller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shiller::Tg(chat_id) => write!(f, "{}", chat_id),
            Shiller::X(account) => write!(f, "{}", account),
        }
    }
}

pub async fn buy_notify(
    chat_name: &str,
    shiller: &Shiller,
    ca: &str,
    client: &Client,
    trenches_chat: &Chat,
    trojan_bot: &Chat,
) -> Result<(), InvocationError> {
    let final_msg = format!(
        "---------------\nChat: {}\n ID: {}\n CA: {}\n---------------",
        chat_name, shiller, ca
    );
    let bullx_link = format!(
        "https://neo.bullx.io/terminal?chainId=1399811149&address={}",
        ca
    );

    #[cfg(not(feature = "remote"))]
    let _ = open_browser(Browser::Brave, &bullx_link);

    {
        let client = client.clone();
        let trenches_chat = trenches_chat.clone();
        let trojan_bot = trojan_bot.clone();
        let final_msg = final_msg.clone();
        let bullx_link = bullx_link.clone();
        tokio::spawn(async move {
            let _ = client
                .send_message(
                    trenches_chat,
                    InputMessage::text(format!("{}\n{}", final_msg, bullx_link)),
                )
                .await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let _ = client.send_message(trojan_bot, "/positions").await;
        });
    }
    Ok(())
}
