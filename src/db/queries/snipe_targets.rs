use crate::{
    db::connect::Database,
    models::{db::DBSnipeTarget, dtos::{CreateSnipeDTO, PatchSnipeTargetDTO}, service::snipe_target::SnipeConfig},
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

    let deactivate_on_snipe = create_snipe_dto.deactivate_on_snipe.unwrap_or(true);

    let is_active = true;

    sqlx::query!(
        r#"
        INSERT INTO snipe_targets (
            target_id, target_name, sol_amount, slippage, priority_fee, deactivate_on_snipe, is_active
        ) VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        target_id,
        target_name,
        snipe_config.sol_amount as f64,
        snipe_config.slippage,
        snipe_config.priority_fee as f64,
        deactivate_on_snipe,
        is_active
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

pub async fn q_patch_snipe_target(
    db: &Database,
    dto: &PatchSnipeTargetDTO,
) -> Result<(), sqlx::Error> {
    let target_id = dto.target_id;

    if let Some(name) = &dto.target_name {
        sqlx::query!(
            r#"
            UPDATE snipe_targets SET target_name = $1 WHERE target_id = $2
            "#,
            name,
            target_id
        )
        .execute(db)
        .await?;
    }

    if let Some(sol) = dto.sol_amount {
        sqlx::query!(
            r#"
            UPDATE snipe_targets SET sol_amount = $1 WHERE target_id = $2
            "#,
            sol as f64,
            target_id
        )
        .execute(db)
        .await?;
    }

    if let Some(slippage) = dto.slippage {
        sqlx::query!(
            r#"
            UPDATE snipe_targets SET slippage = $1 WHERE target_id = $2
            "#,
            slippage,
            target_id
        )
        .execute(db)
        .await?;
    }

    if let Some(priority_fee) = dto.priority_fee {
        sqlx::query!(
            r#"
            UPDATE snipe_targets SET priority_fee = $1 WHERE target_id = $2
            "#,
            priority_fee as f64,
            target_id
        )
        .execute(db)
        .await?;
    }

    if let Some(is_active) = dto.is_active {
        sqlx::query!(
            r#"
            UPDATE snipe_targets SET is_active = $1 WHERE target_id = $2
            "#,
            is_active,
            target_id
        )
        .execute(db)
        .await?;
    }

    if let Some(deactivate_on_snipe) = dto.deactive_on_snipe {
        sqlx::query!(
            r#"
            UPDATE snipe_targets SET deactivate_on_snipe = $1 WHERE target_id = $2
            "#,
            deactivate_on_snipe,
            target_id
        )
        .execute(db)
        .await?;
    }

    Ok(())
}
