use crate::{
    constants::GLOBALY_BLOCKED_CAS, db::queries::snipe_targets::q_patch_snipe_target,
    state::AppState, types::dtos::PatchSnipeTargetDTO,
};
use grammers_client::{Client, InvocationError};
use shared::pf::buy_ca;
use std::sync::Arc;

use crate::sniper::buy_notify;

#[cfg(not(feature = "remote"))]
use shared::utils::play_buy_notif;

pub async fn snipe(
    chat_id: i64,
    _client: &Client,
    shared_state: &Arc<AppState>,
    ca: &str,
) -> Result<(), InvocationError> {
    //TODO: Worst solution but for now it works.FIX ASAP

    if GLOBALY_BLOCKED_CAS.contains(&ca.to_string()) {
        return Ok(());
    }

    let snipe_targets = &shared_state.snipe_targets;
    let snipe_target_option = snipe_targets.get_mut(&chat_id);

    if let Some(mut snipe_target) = snipe_target_option {
        if !snipe_target.is_active {
            return Ok(());
        }
        match buy_ca(
            &shared_state.pf_api_url,
            &snipe_target.snipe_config,
            ca,
            shared_state.priority_fee_multiplier,
            &shared_state.request_client,
        )
        .await
        {
            Ok(_) => {
                if snipe_target.deactivate_on_snipe {
                    snipe_target.is_active = false;

                    let db = shared_state.db.clone();
                    let patch = PatchSnipeTargetDTO {
                        target_id: chat_id,
                        is_active: Some(false),
                        target_name: None,
                        sol_amount: None,
                        slippage: None,
                        priority_fee: None,
                        deactivate_on_snipe: None,
                    };

                    tokio::spawn(async move {
                        let _ = q_patch_snipe_target(&db, &patch).await;
                    });
                }

                // TODO: write to db, needs patch above.
                snipe_target.past_shills.push(ca.to_string());

                #[cfg(not(feature = "remote"))]
                play_buy_notif();

                {
                    let client = _client.clone();
                    let chat_name = snipe_target.target_name.clone();
                    let trenches_chat = shared_state.sniper_trenches_chat.clone();
                    let trojan_bot = shared_state.trojan_bot_chat.clone();
                    let ca = ca.to_owned();

                    tokio::spawn(async move {
                        let _ = buy_notify(
                            &chat_name,
                            &super::Shiller::Tg(chat_id),
                            &ca,
                            &client,
                            &trenches_chat,
                            &trojan_bot,
                        )
                        .await;
                    });
                }
            }
            Err(error) => {
                println!("ERROR: {:?}", error)
            }
        }
    }
    Ok(())
}
