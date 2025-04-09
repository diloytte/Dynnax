use dashmap::DashMap;
use grammers_client::types::Chat;
use grammers_client::{Client, InvocationError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DialogData {
    pub id: i64,
    pub name: String,
    pub dialog_type: u8,
}

impl From<DialogData> for (String,u8){
    fn from(value: DialogData) -> Self {
        (value.name,value.dialog_type)
    }
}



pub async fn get_dialogs(client: &Client) -> Result<Vec<DialogData>, InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    let mut dialogs: Vec<DialogData> = vec![];

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await?;
        if let Some(next_dialog) = next_dialog_option {
            let chat = next_dialog.chat();
            let dialog_type = match chat {
                Chat::User(_) => 0,
                Chat::Group(_) => 1,
                Chat::Channel(_) => 2,
            };
            dialogs.push(DialogData {
                name: chat.name().to_string(),
                id: chat.id(),
                dialog_type,
            });
        }
    }

    Ok(dialogs)
}
