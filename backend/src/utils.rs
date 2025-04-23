use std::{fs::File, io::{self, BufReader}, process::Command, sync::Arc};
use crate::{
    db::queries::{snipe_targets::q_get_all_snipe_targets, x_snipe_targets::q_get_all_x_snipe_targets}, state::AppState,
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
            state.twitter_snipe_targets.insert(x_snipe_name,twitter_snipe_target.into());
        }
    }

    Ok(())
}