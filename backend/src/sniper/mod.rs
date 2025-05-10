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
    client_informer: &Client,
    client: &Client,
    trenches_chat: &Chat,
    trojan_bot: &Chat,
) -> Result<(), InvocationError> {
    let final_msg = format!(
        "---------------\nChat: {}\n ID: {}\n CA: {}\n---------------",
        chat_name, shiller, ca
    );
    let link = format!("https://axiom.trade/t/{}", ca);

    #[cfg(not(feature = "remote"))]
    let _ = open_browser(Browser::Brave, &link);

    {
        let client = client.clone();
        let trenches_chat = trenches_chat.clone();
        let trojan_bot = trojan_bot.clone();
        let final_msg = final_msg.clone();
        let link = link.clone();
        let client_informer_clone = client_informer.clone();

        tokio::spawn(async move {
            let _ = client_informer_clone
                .send_message(
                    trenches_chat,
                    InputMessage::text(format!("{}\n{}", final_msg, link)),
                )
                .await;
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            let _ = client.send_message(trojan_bot, "/positions").await;
        });
    }
    Ok(())
}
