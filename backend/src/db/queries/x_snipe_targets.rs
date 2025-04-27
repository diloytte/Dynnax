use shared::{db::Database, types::SnipeConfig};

use crate::types::{db::DBXSnipeTarget, dtos::snipe_x::PatchXSnipeTargetDTO};

pub async fn q_create_x_snipe_target(
    db: &Database,
    target_name: &str,
    snipe_config: &SnipeConfig,
    deactivate_on_snipe: &Option<bool>,
) -> Result<(), sqlx::error::Error> {
    let deactivate_on_snipe = deactivate_on_snipe.unwrap_or(true);
    let is_active = true;

    sqlx::query!(
        r#"
        INSERT INTO x_snipe_targets (
            target_name, sol_amount, slippage, priority_fee, deactivate_on_snipe, is_active
        ) VALUES ($1, $2, $3, $4, $5, $6)
        "#,
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

pub async fn q_get_all_x_snipe_targets(
    db: &Database,
) -> Result<Vec<DBXSnipeTarget>, sqlx::error::Error> {
    let rows = sqlx::query_as!(
        DBXSnipeTarget,
        r#"
        SELECT id, target_name, sol_amount, slippage, priority_fee, is_active, deactivate_on_snipe
        FROM x_snipe_targets
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(rows)
}

pub async fn q_patch_x_snipe_target(
    db: &Database,
    dto: &PatchXSnipeTargetDTO,
) -> Result<(), sqlx::Error> {
    let name = &dto.target_name;
    sqlx::query!(
        r#"
            UPDATE x_snipe_targets SET target_name = $1 WHERE target_name = $2
            "#,
        name,
        name
    )
    .execute(db)
    .await?;

    if let Some(sol) = dto.sol_amount {
        sqlx::query!(
            r#"
            UPDATE x_snipe_targets SET sol_amount = $1 WHERE target_name = $2
            "#,
            sol as f64,
            name
        )
        .execute(db)
        .await?;
    }

    if let Some(slippage) = dto.slippage {
        sqlx::query!(
            r#"
            UPDATE x_snipe_targets SET slippage = $1 WHERE target_name = $2
            "#,
            slippage,
            name
        )
        .execute(db)
        .await?;
    }

    if let Some(priority_fee) = dto.priority_fee {
        sqlx::query!(
            r#"
            UPDATE x_snipe_targets SET priority_fee = $1 WHERE target_name = $2
            "#,
            priority_fee as f64,
            name
        )
        .execute(db)
        .await?;
    }

    if let Some(is_active) = dto.is_active {
        sqlx::query!(
            r#"
            UPDATE x_snipe_targets SET is_active = $1 WHERE target_name = $2
            "#,
            is_active,
            name
        )
        .execute(db)
        .await?;
    }

    if let Some(deactivate_on_snipe) = dto.deactivate_on_snipe {
        sqlx::query!(
            r#"
            UPDATE x_snipe_targets SET deactivate_on_snipe = $1 WHERE target_name = $2
            "#,
            deactivate_on_snipe,
            name
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

pub async fn q_delete_x_snipe_target(
    db: &Database,
    target_name: &String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM x_snipe_targets WHERE target_name = $1",
        target_name
    )
    .execute(db)
    .await?;
    Ok(())
}
