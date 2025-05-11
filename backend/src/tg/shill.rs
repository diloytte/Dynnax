use std::time::Duration;

use grammers_client::types::Chat;
use grammers_client::{Client, InputMessage, InvocationError};
use tokio::time::sleep;

pub async fn shill_in_groupchats(
    chats: &Vec<Chat>,
    client: &Client,
    message: &String,
    is_test:bool,
    test_chat:&Chat
) -> Result<(), InvocationError> {
    if is_test {
        sleep(Duration::from_secs(1)).await;
        client.send_message(test_chat,InputMessage::text(message)).await?;
        return Ok(())
    }
    for chat in chats {
        client
            .send_message(chat, InputMessage::text(message))
            .await?;
        sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}
