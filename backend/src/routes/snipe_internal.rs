use shared::types::SnipeTarget;

use crate::{
    db::queries::snipe_targets::q_create_snipe_target, state::AppState, types::dtos::CreateSnipeDTO,
};

pub async fn create_snipe_target_internal(
    state: &AppState,
    create_snipe_dto: CreateSnipeDTO,
) -> Result<SnipeTarget, String> {
    if state.all_dialogs.get(&create_snipe_dto.target_id).is_none() {
        return Err(format!(
            "Dialog with ID: {} does not exist.",
            create_snipe_dto.target_id
        ));
    }

    if state
        .snipe_targets
        .get(&create_snipe_dto.target_id)
        .is_some()
    {
        return Err(format!(
            "Snipe Target with ID: {} already exists.",
            create_snipe_dto.target_id
        ));
    }

    if let Err(error) = q_create_snipe_target(&state.db, &create_snipe_dto).await {
        println!("Error <create_snipe_target>: {}", error);
        return Err("Something went wrong.".to_string());
    }

    let snipe_target = SnipeTarget {
        target_name: create_snipe_dto.target_name,
        snipe_config: create_snipe_dto.snipe_config.unwrap_or_default(),
        is_active: true,
        deactivate_on_snipe: create_snipe_dto.deactivate_on_snipe.unwrap_or(true),
        past_shills: vec![],
    };

    state
        .snipe_targets
        .insert(create_snipe_dto.target_id, snipe_target.clone());

    Ok(snipe_target)
}
