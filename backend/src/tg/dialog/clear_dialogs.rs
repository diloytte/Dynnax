use axum::{Extension, http::StatusCode, response::IntoResponse};

use crate::models::other::AppStateExtension;

pub async fn clear_dialogs(Extension(state): AppStateExtension) -> impl IntoResponse {
    let clinet = state.tg_client.as_ref().unwrap();
    let mut iter_dialogs = clinet.iter_dialogs();

    let dialogs_len = iter_dialogs.total().await.unwrap_or(0);

    for _ in 0..dialogs_len {
        let next_dialog_result_option = iter_dialogs.next().await;

        match next_dialog_result_option {
            Ok(next_dialog_option) => {
                if let Some(dialog) = next_dialog_option {
                    let _ = clinet.mark_as_read(dialog.chat).await;
                }
            }
            Err(err) => {
                println!("Error in clearning chats: {:?}", err);
            }
        }
    }

    StatusCode::OK
}
