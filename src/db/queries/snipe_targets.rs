use crate::{
    db::connect::Database,
    models::{db::DBSnipeTarget, dtos::CreateSnipeDTO, service::snipe_target::SnipeConfig},
};

pub async fn q_create_snipe_target(
    db: &Database,
    create_snipe_dto: &CreateSnipeDTO,
) -> Result<(), sqlx::error::Error> {
    let target_name = &create_snipe_dto.target_name;
    let target_id = create_snipe_dto.target_id; // target_id is now passed explicitly
    let snipe_config_option = create_snipe_dto.snipe_config.as_ref();

    let snipe_config = match snipe_config_option {
        Some(snipe_config) => snipe_config,
        None => &SnipeConfig::default(),
    };

    let deactivate_on_snipe = create_snipe_dto.deactivate_on_snipe.unwrap_or(false);

    sqlx::query!(
        r#"
        INSERT INTO snipe_targets (
            target_id, target_name, sol_amount, slippage, priority_fee, deactivate_on_snipe
        ) VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        target_id,
        target_name,
        snipe_config.sol_amount as f64,
        snipe_config.slippage,
        snipe_config.priority_fee as f64,
        deactivate_on_snipe
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn q_get_all_snipe_targets(
    db: &Database,
) -> Result<Vec<DBSnipeTarget>, sqlx::error::Error> {
    let rows = sqlx::query_as!(
        DBSnipeTarget,
        r#"
        SELECT id, target_name, target_id, sol_amount, slippage, priority_fee, is_active, deactivate_on_snipe, past_shills
        FROM snipe_targets
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(rows)
}
