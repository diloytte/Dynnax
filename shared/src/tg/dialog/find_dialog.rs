use grammers_client::Client;
use grammers_client::types::{Chat,Dialog};

pub async fn find_dialog_chat_by_id(client: &Client, id: i64) -> Option<Chat> {
    let mut iter_dialogs = client.iter_dialogs();
    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    for _ in 0..dialogs_len {
        let next_dialog_option = iter_dialogs.next().await.unwrap();
        if let Some(next_dialog) = next_dialog_option {
            if next_dialog.chat.id() == id {
                return Some(next_dialog.chat);
            }
        }
    }

    None
}

pub async fn find_dialog_chat_by_id_from_list(all_dialogs:&Vec<Dialog>, id: i64) -> Option<Chat> {
    for dialog in all_dialogs {
            if dialog.chat.id() == id {
                return Some(dialog.clone().chat);
            }
        }
    None
}
