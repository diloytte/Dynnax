use std::any::Any;

use grammers_client::types::Chat;
use grammers_client::{Client, InvocationError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DialogData {
    pub name: String,
    pub id: i64,
    pub dialog_type:u8
}

pub async fn get_dialogs(
    client: &Client,
) -> Result<Vec<DialogData>, InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    let mut dialogs: Vec<DialogData> = vec![];

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await?;
        if let Some(next_dialog) = next_dialog_option {
            let chat = next_dialog.chat();
            let dialog_type = match chat{
                Chat::User(user) => 0,
                Chat::Group(group) => 1,
                Chat::Channel(channel) => 2,
            };
            dialogs.push(DialogData {
                name: chat.name().to_string(),
                id: chat.id(),
                dialog_type
            });
        }
    }

    Ok(dialogs)
}
