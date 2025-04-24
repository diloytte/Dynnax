use grammers_client::{Client, InvocationError};

pub async fn clear_dialogs(client:&Client) ->Result<(),InvocationError> {
    let mut iter_dialogs = client.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    for _ in 0..dialogs_len {
        let next_dialog_result_option = iter_dialogs.next().await;

        match next_dialog_result_option {
            Ok(next_dialog_option) => {
                if let Some(dialog) = next_dialog_option {
                    let _ = client.mark_as_read(dialog.chat).await;
                }
            }
            Err(err) => {
                println!("Error in clearning chats: {:?}", err);
            }
        }
    }

    Ok(())
}
