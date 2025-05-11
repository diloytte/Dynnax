use grammers_client::{Client, InvocationError};

pub async fn clear_dialogs(
    client: &Client,
    ignore_user: Option<bool>,
) -> Result<(), InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    for _ in 0..dialogs_len {
        let next_dialog_result_option = iter_dialogs.next().await;

        match next_dialog_result_option {
            Ok(next_dialog_option) => {
                if let Some(dialog) = next_dialog_option {
                    let chat = dialog.chat;

                    if ignore_user.is_some() {
                        match chat {
                            grammers_client::types::Chat::User(_) => continue,
                            _ => {} 
                        }
                    }

                    let _ = client.mark_as_read(chat).await;
                }
            }
            Err(err) => {
                println!("Error in clearning chats: {:?}", err);
            }
        }
    }

    Ok(())
}
