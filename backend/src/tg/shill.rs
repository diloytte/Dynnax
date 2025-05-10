use std::time::Duration;

use grammers_client::types::Chat;
use grammers_client::{Client, InputMessage, InvocationError};
use tokio::time::sleep;

pub async fn shill_in_groupchats(
    chats: &Vec<Chat>,
    client: &Client,
    message: &String,
) -> Result<(), InvocationError> {
    for chat in chats {
        client
            .send_message(chat, InputMessage::text(message))
            .await?;
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
