use std::sync::Arc;

use crate::state::AppState;

pub async fn load_snipe_configurations(_state: &Arc<AppState>) -> Result<(), ()> {
    Ok(())
}
