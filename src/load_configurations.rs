use std::sync::Arc;

use crate::{db::queries::snipe_targets::q_get_all_snipe_targets, state::AppState};

pub async fn load_snipe_configurations(state: &Arc<AppState>) -> Result<(), ()> {
    let db = &state.db;
    let snipe_targets_result = q_get_all_snipe_targets(db).await;

    if let Ok(snipe_targets) = snipe_targets_result{
        for snipe_target in snipe_targets {
            state.snipe_targets.insert(snipe_target.target_id,snipe_target.into());
        }
    }

    Ok(())
}
