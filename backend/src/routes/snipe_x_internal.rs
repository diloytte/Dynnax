use shared::types::TwitterTarget;

use crate::{
    db::queries::x_snipe_targets::q_create_x_snipe_target, state::AppState,
    types::dtos::snipe_x::CreateXSnipeTargetDTO,
};

pub async fn create_snipe_x_target_internal(
    state: &AppState,
    dto: CreateXSnipeTargetDTO,
) -> Result<TwitterTarget, String> {
    if state.twitter_snipe_targets.get(&dto.target_name).is_some() {
        return Err(format!(
            "Twitter Snipe Target with Name: {} already exists.",
            dto.target_name
        ));
    }

    let twitter_target_name = &dto.target_name;
    let twitter_snipe_config = dto.snipe_config.unwrap_or_default();

    if let Err(error) = q_create_x_snipe_target(
        &state.db,
        twitter_target_name,
        &twitter_snipe_config,
        &dto.deactivate_on_snipe,
        dto.is_active,
    )
    .await
    {
        println!("Error <create_snipe_x_target>: {}", error);
        return Err("Something went wrong.".to_string());
    }

    let twitter_snipe_target = TwitterTarget {
        target_name: twitter_target_name.clone(),
        snipe_config: twitter_snipe_config,
        is_active: true,
        deactivate_on_snipe: dto.deactivate_on_snipe.unwrap_or(true),
    };

    state.twitter_snipe_targets.insert(
        twitter_target_name.to_string(),
        twitter_snipe_target.clone(),
    );

    Ok(twitter_snipe_target)
}
