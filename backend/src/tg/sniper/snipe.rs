use crate::{constants::GLOBALY_BLOCKED_CAS, db::queries::snipe_targets::q_patch_snipe_target, types::dtos::PatchSnipeTargetDTO, pf::buy_ca, state::AppState};
use grammers_client::{Client, InputMessage, InvocationError};
use shared::{types::Browser, utils::{open_browser, play_buy_notif}};
use std::sync::Arc;

pub async fn snipe(
    chat_id: i64,
    client: &Client,
    shared_state: &Arc<AppState>,
    ca: &String,
) -> Result<(), InvocationError> {

    //TODO: Worst solution but for now it works.FIX ASAP
    if GLOBALY_BLOCKED_CAS.contains(ca){
        return Ok(());
    }

    let snipe_targets = &shared_state.snipe_targets;
    let snipe_target_option = snipe_targets.get_mut(&chat_id);

    if let Some(mut snipe_target) = snipe_target_option {
        if !snipe_target.is_active{
            return Ok(());
        }
        match buy_ca(&shared_state.pf_api_url, &snipe_target, &ca,shared_state.priority_fee_multiplier).await {
            Ok(_) => {
                play_buy_notif();
                if snipe_target.deactivate_on_snipe {
                    snipe_target.is_active = false;
                }
                    //TODO: DB Write too!
                snipe_target.past_shills.push(ca.to_string());
                q_patch_snipe_target(&shared_state.db, &PatchSnipeTargetDTO {
                    target_id:chat_id,
                    is_active:Some(false),
                    target_name:None,
                    sol_amount:None,
                    slippage:None,
                    priority_fee:None,
                    deactive_on_snipe:None
                });
                let chat_name = &snipe_target.target_name;
                let final_msg = format!(
                    "---------------\nChat: {}\n ID: {}\n CA: {}\n---------------",
                    chat_name, chat_id, ca
                );
                println!("{}", final_msg);
                let bullx_link = format!(
                    "https://neo.bullx.io/terminal?chainId=1399811149&address={}",
                    ca
                );
                let _ = open_browser(
                    Browser::Brave,
                    &bullx_link,
                );
                // TODO: Cant be send from me, it wont be receiving message notifications.
                let trenches_chat = &shared_state.sniper_trenches_chat;
                client.send_message(trenches_chat,InputMessage::text(format!("{}\n{}",final_msg,bullx_link))).await?;
            }
            Err(error) => {
                println!("ERROR: {:?}", error)
            }
        }
    }
    Ok(())
}
