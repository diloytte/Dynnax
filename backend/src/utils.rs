use std::{sync::Arc, vec};

use grammers_client::{Client, InvocationError, types::{Chat,Dialog}};
use shared::tg::dialog::find_dialog::find_dialog_chat_by_id_from_list;

use crate::{
    db::queries::{
        snipe_targets::q_get_all_snipe_targets, x_snipe_targets::q_get_all_x_snipe_targets,
    },
    state::AppState,
};

pub async fn load_snipe_configurations(state: &Arc<AppState>) -> Result<(), ()> {
    let db = &state.db;
    let snipe_targets_result = q_get_all_snipe_targets(db).await;

    if let Ok(snipe_targets) = snipe_targets_result {
        for snipe_target in snipe_targets {
            state
                .snipe_targets
                .insert(snipe_target.target_id, snipe_target.into());
        }
    }

    let twitter_snipe_targets_result = q_get_all_x_snipe_targets(db).await;

    if let Ok(twitter_snipe_targets) = twitter_snipe_targets_result {
        for twitter_snipe_target in twitter_snipe_targets {
            let x_snipe_name = twitter_snipe_target.target_name.clone();
            state
                .twitter_snipe_targets
                .insert(x_snipe_name, twitter_snipe_target.into());
        }
    }

    Ok(())
}


//TODO: Load from db, make endpoints for add/remove
pub async fn load_shill_groups(ids_from_env_raw:&String,dialogs:&Vec<Dialog>) -> (Vec<Chat>, Vec<String>) {
    let ids = parse_group_ids(ids_from_env_raw);
    let mut chats: Vec<Chat> = vec![];
    let mut errors: Vec<String> = vec![];

    for id in ids {
        match find_dialog_chat_by_id_from_list(dialogs, id).await {
            Some(chat) => chats.push(chat),
            None => errors.push(format!("Couldn't find chat with ID: {}", id)),
        }
    }

    (chats, errors)
}


fn parse_group_ids(ids_from_env_raw: &str) -> Vec<i64> {
    ids_from_env_raw
        .split(',')
        .map(|s| s.trim().parse::<i64>().expect("Invalid i64"))
        .collect()
}


pub async fn add_one_time_snipe_x_target(
    client: &Client,
    redacted_chat: &Chat,
    target_id: &str,
) -> Result<(), InvocationError> {
    client
        .send_message(redacted_chat, format!("/add {}", target_id))
        .await?;
    Ok(())
}

pub async fn remove_one_time_snipe_x_target(
    client: &Client,
    redacted_chat: &Chat,
    target_id: &str,
) -> Result<(), InvocationError> {
    client
        .send_message(redacted_chat, format!("/remove {}", target_id))
        .await?;
    Ok(())
}
