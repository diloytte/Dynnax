use grammers_client::types::Chat;
use grammers_client::types::Dialog;
use grammers_client::{Client, InvocationError};
use serde::Serialize;

pub async fn get_dialogs(client: &Client) -> Result<Vec<Dialog>, InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    let mut dialogs: Vec<Dialog> = vec![];

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await?;
        if let Some(next_dialog) = next_dialog_option {
            dialogs.push(next_dialog);
        }
    }

    Ok(dialogs)
}

pub fn get_dialog_type_as_number(dialog: &Dialog) -> u8 {
    match dialog.chat {
        Chat::User(_) => 0,
        Chat::Group(_) => 1,
        Chat::Channel(_) => 2,
    }
}

#[derive(Debug, Serialize)]
pub struct SimplifiedDialog {
    pub id: i64,
    pub name: String,
    #[serde(rename = "dialogType")]
    pub dialog_type: u8,
}

impl From<SimplifiedDialog> for (String, u8) {
    fn from(value: SimplifiedDialog) -> Self {
        (value.name, value.dialog_type)
    }
}

pub fn simplify_dialog(dialog: &Dialog) -> SimplifiedDialog {
    SimplifiedDialog {
        id: dialog.chat.id(),
        name: dialog.chat.name().to_string(),
        dialog_type: get_dialog_type_as_number(dialog),
    }
}
