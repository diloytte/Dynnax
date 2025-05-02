use crate::db::queries::x_snipe_targets::q_patch_x_snipe_target;
use crate::state::AppState;
use crate::types::dtos::snipe_x::PatchXSnipeTargetDTO;
use crate::utils::remove_one_time_snipe_x_target;
use grammers_client::types::Message;
use grammers_client::{Client, InvocationError};
use shared::pf::buy_ca;
use shared::twitter_regex::extract_twitter_sender;
#[cfg(not(feature = "remote"))]
use shared::utils::play_buy_notif;
use std::sync::Arc;

use super::buy_notify;

pub async fn snipe_x(
    message: &Message,
    #[cfg_attr(not(feature = "remote"), allow(unused_variables))] _client: &Client,
    shared_state: &Arc<AppState>,
    ca: &str,
) -> Result<(), InvocationError> {
    let sender_option = extract_twitter_sender(message.text());

    if sender_option.is_none() {
        println!("Unable to find Twitter sender.");
        println!("Message:");
        println!("{}", message.text());
        println!("---------------");
        return Ok(());
    }

    let twitter_snipe_targets = &shared_state.twitter_snipe_targets;

    let twitter_sender = sender_option.unwrap();
    let twitter_snipe_target_option = twitter_snipe_targets.get_mut(&twitter_sender);

    if twitter_snipe_target_option.is_none() {
        println!("{} is not a Twitter Snipe Target.", twitter_sender);
        return Ok(());
    }

    let mut twitter_snipe_target = twitter_snipe_target_option.unwrap();

    if !twitter_snipe_target.is_active {
        println!("{} is not a Active.", twitter_sender);
        return Ok(());
    }

    match buy_ca(
        &shared_state.pf_api_url,
        &twitter_snipe_target.snipe_config,
        ca,
        1,
        &shared_state.request_client,
    )
    .await
    {
        Ok(_) => {
            if twitter_snipe_target.deactivate_on_snipe {
                twitter_snipe_target.is_active = false;

                let db = shared_state.db.clone();
                let patch = PatchXSnipeTargetDTO {
                    target_name: twitter_snipe_target.target_name.clone(),
                    sol_amount: None,
                    slippage: None,
                    priority_fee: None,
                    is_active: Some(false),
                    deactivate_on_snipe: Some(true),
                };
                tokio::spawn(async move {
                    let _ = q_patch_x_snipe_target(&db, &patch).await;
                });
            }

            #[cfg(not(feature = "remote"))]
            play_buy_notif();
            {
                let client = _client.clone();
                let chat_name = twitter_snipe_target.target_name.clone();
                let trenches_chat = shared_state.sniper_trenches_chat.clone();
                let trojan_bot = shared_state.trojan_bot_chat.clone();
                let ca = ca.to_owned();
                let informer = shared_state.tg_client_informer.clone(); // clone here

                tokio::spawn(async move {
                    let _ = buy_notify(
                        &chat_name,
                        &super::Shiller::X(twitter_sender),
                        &ca,
                        &informer,
                        &client,
                        &trenches_chat,
                        &trojan_bot,
                    )
                    .await;
                });
            }

            if twitter_snipe_target.is_one_time {
                //TODO: Remove from db too
                remove_one_time_snipe_x_target(
                    &shared_state.tg_client,
                    &shared_state.redacted_bot_chat,
                    &twitter_snipe_target.target_name,
                )
                .await?;
            }
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }

    Ok(())
}
