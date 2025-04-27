use dotenv::dotenv;
use grammers_client::{Client, Update};
use shared::{
    pf::buy_ca_tg,
    tg::{client::connect_client, dialog::find_dialog::find_dialog_chat_by_id},
    twitter_regex::extract_twitter_sender,
};
use std::{collections::HashMap, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client: Client = connect_client("./dead_x_sniper/session.session").await?;

    let redacted_self_bot_father_dialog_id: i64 =
        env::var("REDACTED_SELF_BOT_FATHER_DIALOG_ID")?.parse()?;

    let redacted_bot_dialog_id: i64 = env::var("REDACTED_SYSTEMS_BOT_DIALOG_ID")?.parse()?;

    let pepeboost_dialog_id: i64 = env::var("PEPEBOOST_DIALOG_ID")?.parse()?;

    let trojan_dialog_id: i64 = env::var("TROJAN_DIALOG_ID")?.parse()?;

    let redacted_bot_dialog = find_dialog_chat_by_id(&client, redacted_bot_dialog_id).await;

    if redacted_bot_dialog.is_none() {
        panic!("Redacted bot does not exist");
    }

    let pepeboost_chat = find_dialog_chat_by_id(&client, pepeboost_dialog_id).await;

    if pepeboost_chat.is_none() {
        panic!("Pepeboost does not exist");
    }

    let trojan_chat = find_dialog_chat_by_id(&client, trojan_dialog_id).await;

    if trojan_chat.is_none() {
        panic!("Trojan does not exist");
    }

    let _trojan_chat = trojan_chat.unwrap();

    let pepeboost_chat = pepeboost_chat.unwrap();

    let redacted_bot_chat = redacted_bot_dialog.unwrap();

    let mut tracked_twitter_account: HashMap<&str, &str> = HashMap::new();

    //TODO: make this dynamix and accessable elsewhere. so u dont have to restart the app once this changes
    tracked_twitter_account.insert(
        "TheRoaringKitty",
        "EKEWAk7hfnwfR8DBb1cTayPPambqyC7pwNiYkaYQKQHp",
    );
    tracked_twitter_account.insert(
        "unity_on_solana",
        "EdhTCqUxXRWQcUd5Fonyz9rapHAB6mABAuVkmPrtpump",
    );

    for tracked in &tracked_twitter_account {
        client
            .send_message(&redacted_bot_chat, format!("/add {}", tracked.0))
            .await?;
    }

    loop {
        match client.next_update().await {
            Ok(Update::NewMessage(message)) => {
                let message_sender = message.sender();

                if message_sender.is_none() {
                    continue;
                }

                let message_sender = message_sender.unwrap();
                let message_sender_id = message_sender.id();
                if message_sender_id != redacted_self_bot_father_dialog_id {
                    continue;
                }

                let message_text = message.text();

                let twitter_sender = extract_twitter_sender(message_text);
                if twitter_sender.is_none() {
                    continue;
                }

                let twitter_sender = twitter_sender.unwrap();

                let ca = tracked_twitter_account.get(twitter_sender.as_str());

                if ca.is_none() {
                    continue;
                }

                let ca = ca.unwrap();

                buy_ca_tg(&client, ca, &pepeboost_chat).await?;

                tracked_twitter_account.remove(twitter_sender.as_str());
            }
            Err(e) => eprintln!("Error in listen_for_updates: {}", e),
            _ => continue,
        }
    }
}
