use std::{io, process::Command, sync::Arc};

use crate::{
    db::queries::snipe_targets::q_get_all_snipe_targets, models::other::Browser, state::AppState,
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

    Ok(())
}

#[macro_export]
macro_rules! json_error {
    ($message:expr) => {
        serde_json::json!({ "error": $message }).to_string()
    };
}

pub fn open_browser(browser: Browser, url: String) -> io::Result<()> {
    match browser {
        Browser::Brave => {
            Command::new("brave-browser")
                .arg("--new-tab")
                .arg(url)
                .spawn()?;
        }
    }

    Ok(())
}
